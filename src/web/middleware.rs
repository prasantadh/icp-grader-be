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
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::RETRYABLE_WRITE_ERROR;
use oauth2::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::User;
// use crate::schema::{get, list, User};
use crate::{config, AppState, Context, Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // TODO: may be these do not need to be public,
    // and i just need a new() function
    pub user_id: i32,
    pub exp: usize,
}

pub async fn resolve_ctx(
    State(state): State<AppState>,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response> {
    use crate::schema::users::dsl::*;

    let bearer = match bearer {
        None => return Ok(next.run(request).await),
        Some(TypedHeader(Authorization(v))) => v,
    };

    let token = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(config().JWT_SIGNING_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::MiscError)?;

    let connection = state.db_pool.get().await.unwrap();
    let user = connection
        .interact(move |c| {
            // something
            users
                .filter(id.eq(token.claims.user_id))
                .select(User::as_select())
                .first(c)
        })
        .await
        .unwrap()
        .unwrap();

    let context = Context::new(user.id, user.role);
    request
        .extensions_mut()
        .insert::<Result<Context>>(Ok(context));
    Ok(next.run(request).await)
}

// Context extractor
// FIXME feels like may be this should be in ctx.rs
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(Error::UnauthorizedActionError)?
            .clone()
    }
}
