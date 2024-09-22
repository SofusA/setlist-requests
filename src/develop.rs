use crate::app::AppState;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

pub fn develop_routes() -> Router<Arc<AppState>> {
    Router::new().route("/develop/create_setlist", get(create_setlist))
}

async fn create_setlist(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state
        .database
        .add_song(
            "Me & My",
            "Dubidub",
            Some("Svedig sang der hangler om dubber. En god gammel banger om gode gamle dubber"),
        )
        .await
        .unwrap();

    state
        .database
        .add_song("Blå Øjne", "Romeo", None)
        .await
        .unwrap();
}
