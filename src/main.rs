use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
};
use futures::{sink::SinkExt, stream::StreamExt};
use setlist_requests::{html, view::View};
use std::sync::Arc;
use tokio::sync::watch::{channel, Receiver, Sender};
use tower_http::{services::ServeDir, trace};
use tracing::info;

pub struct AppState {
    rx: Receiver<Message>,
    pub tx: Sender<Message>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    let (tx, rx) = channel(Message::Text("{}".to_string()));
    let shared_state = Arc::new(AppState { rx, tx });

    let mut assets_path = std::env::current_dir().unwrap();
    assets_path.push("assets");

    let router = axum::Router::new()
        .route("/", axum::routing::get(keys))
        .route("/api/smoke", get(smoke_test))
        .route("/websocket", get(websocket_handler))
        .with_state(shared_state)
        .nest_service("/assets", ServeDir::new(assets_path.to_str().unwrap()))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::DEBUG)),
        );

    let address = "0.0.0.0:3000";
    tracing::info!("listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn keys() -> View {
    let test = html! { <div class="flex flex justify-center w-full">hello</div> };

    page(test, "lol")
}

fn page(component: View, title: &str) -> View {
    let style_url = "/assets/styles.css?version=1";
    let doctype = "<!DOCTYPE html>";

    html! {
        {doctype}

        <html lang="en" class="h-full dark">
            <head>
                <title>{title}</title>
                <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎵</text></svg>">
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link rel="stylesheet" href=style_url />
                <script src="https://unpkg.com/htmx.org@2.0.0"></script>
            </head>
            <body
                class="h-full text-black bg-white dark:text-white dark:bg-slate-800"
                hx-history="false"
            >
                <div class="flex overscroll-none flex-col h-full">{component}</div>

                <script src="/assets/scripts/htmx-config.js?version=1"></script>

                {if cfg!(debug_assertions) {
                    html! { <script src="/assets/scripts/develop-updates.js"></script> }
                } else {
                    Default::default()
                }}

            </body>
        </html>
    }
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
