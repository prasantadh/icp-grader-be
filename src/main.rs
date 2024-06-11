mod error;
mod schema;
mod web;

use axum::Router;
pub use error::{Error, Result};
use mongodb::{Client, Database};
use tokio::net::TcpListener;

use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn_str = env::var("MONGO_CONN_URI")?;
    let client = Client::with_uri_str(conn_str).await?;
    let db = client.database(env::var("DB_NAME")?.as_ref());
    if cfg!(debug_assertions) {
        schema::init(&db).await?;
    }

    let app_state = AppState { db };

    let routes = Router::new().merge(web::group::routes(app_state.clone()));
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(tcp_listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
