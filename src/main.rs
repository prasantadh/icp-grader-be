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

#[tokio::main]
async fn main() -> Result<()> {
    let conn_str = &config().MONGO_CONN_URI;
    let client = Client::with_uri_str(conn_str).await.unwrap();
    let db = client.database(&config().DB_NAME);
    if cfg!(debug_assertions) {
        schema::init(&db).await?;
    }

    let app_state = AppState { db };

    let routes = Router::new()
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
        .layer(CookieManagerLayer::new());

    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(tcp_listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
