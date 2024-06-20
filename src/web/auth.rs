use std::collections::HashMap;

use axum::{
    extract::{Host, Query, State},
    response::Redirect,
    routing::get,
    Json, Router,
};
use mongodb::bson::doc;
use oauth2::{
    basic::BasicClient,
    reqwest::{async_http_client, http_client},
    revocation, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::{
    config,
    schema::{self, User},
    AppState, Error, Result,
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/google/auth", get(login_handler))
        .route("/google/auth_return", get(login_return_handler))
        .with_state(state)
}

fn get_oauth_client() -> Result<BasicClient> {
    let google_client_id = ClientId::new(config().GOOGLE_OAUTH_CLIENT.clone());
    let google_client_secret = ClientSecret::new(config().GOOGLE_OAUTH_SECRET.clone());
    let auth_url = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url).map_err(|_| Error::MiscError)?;
    let token_url = String::from("https://www.googleapis.com/oauth2/v3/token");
    let token_url = TokenUrl::new(token_url).map_err(|_| Error::MiscError)?;
    let redirect_url = config().GOOGLE_OAUTH_RETURN.clone();
    let redirect_url = RedirectUrl::new(redirect_url).map_err(|_| Error::MiscError)?;
    let revocation_url = String::from("https://oauth2.googleapis.com/revoke");
    let revocation_url = RevocationUrl::new(revocation_url).map_err(|_| Error::MiscError)?;

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url)
    .set_revocation_uri(revocation_url);

    Ok(client)
}

// TODO eventually will have to insert data as extension middleware but for now let it be
pub async fn login_handler(State(state): State<AppState>) -> Result<Redirect> {
    let client = get_oauth_client()?;
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn login_return_handler(
    State(state): State<AppState>,
    Query(mut params): Query<HashMap<String, String>>,
    Host(hostname): Host,
) -> Result<Json<String>> {
    // extract oauth state and code
    let oauth_state = CsrfToken::new(params.remove("state").ok_or(Error::OauthError)?);
    let code = AuthorizationCode::new(params.remove("code").ok_or(Error::OauthError)?);

    // TODO verify the csrf token along with pkce

    // exchange code with a token
    let client = get_oauth_client()?;
    let token = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .map_err(|_| Error::OauthExchangeCodeError)?;

    // get userinfo from google
    let client = reqwest::Client::new();
    let user_data = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|_| Error::OauthError)?
        .text()
        .await
        .map_err(|_| Error::OauthError)?;
    // TODO this should probably be a SerdeJsonSerializationError eventually
    let data: UserData =
        serde_json::from_str(user_data.as_str()).map_err(|_| Error::MongoSerializationError)?;
    // at this point, we make sure this user has access to your services
    // then return a token that they can use.
    let users = schema::list::<User>(&state.db, doc! {"email": data.email}).await?;
    let user = users.first().ok_or(Error::UserIdIsNullError)?;
    let token = schema::get_token(user.id().ok_or(Error::RecordNotFound)?).await?;
    Ok(Json(token))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
}
