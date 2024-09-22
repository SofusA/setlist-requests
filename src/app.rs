use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use std::{env, sync::Arc};
use tokio::sync::watch::{channel, Receiver, Sender};
use tower_http::{services::ServeDir, trace};
use tracing::info;

use crate::{
    database::{Credentials, Database},
    develop::develop_routes,
    html,
    page::page,
    setlist::{add_song, setlist_page},
    view::View,
};

pub struct AppState {
    rx: Receiver<Message>,
    pub tx: Sender<Message>,
    pub database: Database,
}

pub fn get_credentials() -> Credentials {
    dotenv().ok();

    let hostname = env::var("DATABASE_HOSTNAME").unwrap();
    let secret = env::var("DATABASE_SECRET").unwrap();
    let user = env::var("DATABASE_USER").unwrap();
    let database = env::var("DATABASE_NAME").unwrap();

    Credentials {
        secret,
        hostname,
        user,
        database,
    }
}

pub async fn serve_app() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    let router = create_router().await;

    let address = "0.0.0.0:3000";
    tracing::info!("listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn create_router() -> Router {
    let credentials = get_credentials();

    let database = Database::new(credentials).await;

    let (tx, rx) = channel(Message::Text("{}".to_string()));
    let shared_state = Arc::new(AppState { rx, tx, database });

    let mut assets_path = std::env::current_dir().unwrap();
    assets_path.push("assets");

    let mut router = axum::Router::new()
        .route("/", get(root))
        .route("/setlist", get(setlist_page).post(add_song))
        .route("/api/smoke", get(smoke_test))
        .route("/websocket", get(websocket_handler));

    if cfg!(debug_assertions) {
        router = router.nest("/", develop_routes())
    }

    router = router
        .nest_service("/assets", ServeDir::new(assets_path.to_str().unwrap()))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
        );

    router.with_state(shared_state)
}

async fn root() -> View {
    let hello = html! {
        <div class="flex justify-center w-full">
            <span>Hello</span>
        </div>
    };

    page(hello, "FestOrkestret Setlist")
}

async fn smoke_test() -> impl IntoResponse {
    "Ok"
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    info!("New client!");
    let (mut sender, _) = socket.split();

    let mut rx = state.rx.clone();

    tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            let msg = rx.borrow().clone();

            if sender.send(msg).await.is_err() {
                info!("Client connection ended");
                break;
            }
        }
    });
}
