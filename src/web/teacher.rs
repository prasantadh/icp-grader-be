use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::Teacher;
use crate::schema::{create, delete, list, update};
use crate::AppState;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/teachers",
            http_post(create::<Teacher>).get(list::<Teacher>),
        )
        .route("/teachers/:id", http_patch(update::<Teacher>))
        .route("/teachers/:id", http_delete(delete::<Teacher>))
        .with_state(state)
}
