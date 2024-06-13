use std::{env::VarError, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    ReadEnvError(VarError),
    MongoError(mongodb::error::Error),
    MiscError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:>}")
    }
}

impl std::error::Error for Error {}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error::ReadEnvError(value)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::MongoError(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
