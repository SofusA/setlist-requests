use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BadRequestError {
    #[error("Too many votes")]
    TooManyVotes,
}
impl IntoResponse for BadRequestError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Bad request: {}", self),
        )
            .into_response()
    }
}
