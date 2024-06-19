use std::{collections::HashMap, env};

use axum::{
    extract::{Host, Query, State},
    response::Redirect,
    routing::get,
    Json, Router,
};
use oauth2::{
    basic::BasicClient, revocation, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

use crate::{config, AppState, Error, Result};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/google/login", get(login_handler))
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
) -> Result<Json<()>> {
    println!("{params:?}");
    todo!()
}
