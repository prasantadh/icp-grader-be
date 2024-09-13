mod config;
mod ctx;
mod error;
mod models;
mod schema;
mod web;

use core::panic;

pub use config::config;
pub use ctx::Context;
pub use error::{Error, Result};

use axum::{middleware, Router};
use deadpool_diesel::postgres::{Manager, Pool};
use diesel::{RunQueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
pub struct AppState {
    db_pool: Pool,
}

pub async fn app() -> Router {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("setting up a connection pool");
    let manager = Manager::new(&config().DATABASE_URL, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .build()
        .expect("Error setting up a connection pool");
    let app_state = AppState { db_pool: pool };

    info!("running pending migrations");
    let connection = app_state
        .db_pool
        .get()
        .await
        .expect("Error getting a connection from pool");

    // TODO: this code is detected as invalid but is valid
    // as verified by running axum diesel-postgres example
    connection
        .interact(|c| c.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();

    {
        use crate::models::{NewUser, Role};
        use crate::schema::users;
        let user = NewUser {
            email: config().ADMIN_EMAIL.clone(),
            role: Role::Admin,
        };

        match connection
            .interact(|c| {
                diesel::insert_into(users::table)
                    .values(user)
                    .returning(crate::models::User::as_returning())
                    .get_result(c)
            })
            .await
            .unwrap()
        {
            Ok(_) => info!("admin created."),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => info!("admin exists, skipping admin setup"),
            Err(e) => panic!("{:?}", e),
        }
    }

    Router::new()
        .merge(web::teacher::routes(app_state.clone()))
        .merge(web::auth::routes(app_state.clone()))
        // TODO: may be rename middleware to just mw as axum uses middleware
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::web::middleware::resolve_ctx,
        ))
        .layer(CookieManagerLayer::new())
        // FIXME: eventually do not allow all the methods and all the origins
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
}

#[tokio::main]
async fn main() -> Result<()> {
    // do all the migrations and setup an admin who will run the app
    {
        // setup and admin user in this block,
        // an admin user is part of the configuration deal
    }
    let app = app().await;
    let tcp_listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
