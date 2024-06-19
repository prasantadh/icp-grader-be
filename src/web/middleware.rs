use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::{self, Parts};
use axum::middleware::Next;
use axum::{async_trait, RequestExt, RequestPartsExt};
use axum::{extract::Request, response::Response};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::schema::{get, User};
use crate::{config, AppState, Context, Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // TODO may be these do not need to be public,
    // and i just need a new() function
    pub user_id: ObjectId,
    pub exp: usize,
}

pub async fn resolve_ctx(
    State(state): State<AppState>,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response> {
    let bearer = match bearer {
        None => return Ok(next.run(request).await),
        Some(TypedHeader(Authorization(v))) => v,
    };

    let token = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(config().JWT_SIGNING_SECRET.as_ref()),
        &Validation::default(),
    )
    .unwrap();

    let user = get::<User>(&state.db, token.claims.user_id)
        .await
        .map_err(|e| Error::RecordNotFound)?;
    let context = Context::new(user.id().ok_or(Error::UserIdIsNullError)?, user.role);
    request
        .extensions_mut()
        .insert::<Result<Context>>(Ok(context));
    Ok(next.run(request).await)
}

// Context extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts.extensions.get::<Result<Context>>().unwrap().clone()
    }
}
