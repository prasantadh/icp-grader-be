mod config;
mod ctx;
mod error;
mod schema;
mod web;

use axum::{middleware, Router};
use mongodb::{Client, Database};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

pub use config::config;
pub use ctx::Context;
pub use error::{Error, Result};

#[derive(Debug, Clone)]
pub struct AppState {
    db: Database,
}

pub async fn app() -> Router {
    let conn_str = &config().MONGO_CONN_URI;
    let client = Client::with_uri_str(conn_str).await.unwrap();
    let db = client.database(&config().DB_NAME);
    let app_state = AppState { db };

    if cfg!(debug_assertions) {
        // FIXME an admin should still probably still be created
        // even if we are in the release version
        schema::init(&app_state.db).await.unwrap();
    }

    Router::new()
        .merge(web::assessment::routes(app_state.clone()))
        .merge(web::teacher::routes(app_state.clone()))
        .merge(web::student::routes(app_state.clone()))
        .merge(web::subject::routes(app_state.clone()))
        .merge(web::submission::routes(app_state.clone()))
        .merge(web::auth::routes(app_state.clone()))
        // TODO may be rename middleware to just mw as axum uses middleware
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::web::middleware::resolve_ctx,
        ))
        .layer(CookieManagerLayer::new())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = app().await;
    let tcp_listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
