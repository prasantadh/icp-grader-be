use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use deadpool_diesel::{postgres::PoolError, InteractError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    // Deadpool error
    Deadpool(String),
    ReadEnvError(String),
    // Mongo Errors
    MongoError(mongodb::error::Error),
    MongoSerializationError,
    // database query errors
    RecordNotFound,
    UserIdIsNullError,
    ContextNotInExtError,
    JWTDecodeError,
    AuthError,
    MiscError,
    // database action errors
    UnauthorizedActionError,
    // OAUTH_ERROR
    OauthError,
    OauthExchangeCodeError,
    TokioSpawnBlockingError,
    OauthUserInfoQueryFailed,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:>}")
    }
}

impl std::error::Error for Error {}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::MongoError(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // whatever the error, currently gets mapped to internal server error
        // TODO use tracing instead of println here
        println!("{self:?}");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::Deadpool(format!("{:?}", value))
    }
}

impl From<PoolError> for Error {
    fn from(value: PoolError) -> Self {
        Error::Deadpool(format!("{:?}", value))
    }
}

impl From<InteractError> for Error {
    fn from(value: InteractError) -> Self {
        Error::Deadpool(format!("{value:?}"))
    }
}
