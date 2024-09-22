use setlist_requests::app::serve_app;

#[tokio::main]
async fn main() {
    serve_app().await;
}
