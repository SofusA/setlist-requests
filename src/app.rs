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
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use random_string::{charsets, generate};
use std::{env, sync::Arc};
use tokio::sync::watch::{channel, Receiver, Sender};
use tower_http::{services::ServeDir, trace};
use tracing::info;

use crate::{
    database::{Credentials, Database},
    html,
    page::page,
    setlist::{add_song, clear_votes, delete_song, setlist_page},
    view::View,
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

    let hostname = env::var("DATABASE_HOSTNAME").unwrap_or("localhost".to_string());
    let secret = env::var("DATABASE_SECRET").unwrap_or("postgres".to_string());
    let user = env::var("DATABASE_USER").unwrap_or("postgres".to_string());
    let database = env::var("DATABASE_NAME").unwrap_or("postgres".to_string());
    let port = env::var("DATABASE_PORT")
        .unwrap()
        .parse::<i32>()
        .unwrap_or(5432);

    Credentials {
        secret,
        hostname,
        user,
        database,
        port,
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

    let router = axum::Router::new()
        .route("/", get(index))
        .route("/vote", get(vote_songs))
        .route("/setlist", get(setlist_page).post(add_song))
        .route("/setlist/:id", delete(delete_song))
        .route("/setlist/votes/clear", post(clear_votes))
        .route("/vote/results", get(vote_result_page))
        .route("/vote/:song_id", post(vote_for_song).delete(delete_vote))
        .route("/api/smoke", get(smoke_test))
        .route("/websocket", get(websocket_handler))
        .nest_service("/assets", ServeDir::new(assets_path.to_str().unwrap()))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
        )
        .layer(middleware::from_fn(remember_me));

    router.with_state(shared_state)
}

async fn index() -> View {
    let index = html! {
        <div class="flex flex-col gap-4">
            <h1 class="text-lg">Stem på hvilke sange FestOrkestret spiller efter pausen</h1>

            <a
                class="p-2 text-lg text-center text-white bg-blue-500 rounded transition-colors hover:bg-blue-400"
                href="/vote"
            >
                Stem her
            </a>
        </div>
    };

    page(index, "Festorkestret Setlist")
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
        let mut cookie = Cookie::new("session_id", generate(6, charsets::ALPHA));
        cookie.set_same_site(SameSite::Strict);
        jar = jar.add(cookie);
    }
    (jar, next.run(request).await)
}
