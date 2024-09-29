use axum::{
    extract::{
        ws::{Message, WebSocket},
        Request, State, WebSocketUpgrade,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use random_string::{charsets, generate};
use std::{env, sync::Arc};
use tokio::sync::watch::{channel, Receiver, Sender};
use tower_http::{services::ServeDir, trace};
use tracing::info;

use crate::{
    database::{Credentials, Database},
    setlist::{add_song, clear_votes, delete_song, setlist_page},
    vote::{delete_vote, vote_for_song, vote_songs},
    vote_results::vote_result_page,
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
        .route("/", get(vote_songs))
        .route("/setlist", get(setlist_page).post(add_song))
        .route("/setlist/:id", delete(delete_song))
        .route("/setlist/votes/clear", post(clear_votes))
        .route("/vote/results", get(vote_result_page))
        .route("/vote/:song_id", post(vote_for_song).delete(delete_vote))
        .route("/api/smoke", get(smoke_test))
        .route("/websocket", get(websocket_handler));

    router = router
        .nest_service("/assets", ServeDir::new(assets_path.to_str().unwrap()))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
        )
        .layer(middleware::from_fn(remember_me));

    router.with_state(shared_state)
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

async fn remember_me(mut jar: CookieJar, request: Request, next: Next) -> (CookieJar, Response) {
    if jar.get("session_id").is_none() {
        jar = jar.add(Cookie::new("session_id", generate(6, charsets::ALPHA)));
    }
    (jar, next.run(request).await)
}
