use setlist_requests::{html, view::View};
use tower_http::trace;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    let router = axum::Router::new()
        .route("/", axum::routing::get(keys))
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
    html! { <div class="flex flex justify-center w-full">hello</div> }
}
