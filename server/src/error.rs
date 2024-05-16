use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DefaultError,
    SerdeError,
    IOError,
    UnknownError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - {:?}\n", "INTO_RES", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        error!("{:<12} - {:?}\n", "FROM_SERDE", error);
        Error::SerdeError
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        error!("{:<12} - {:?}\n", "FROM_IO", error);
        Error::IOError
    }
}
