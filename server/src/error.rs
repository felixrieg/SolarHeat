use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DefaultError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - {:?}\n", "INTO_RES", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
