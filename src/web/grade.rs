use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::Grade;
use crate::schema::{create, delete, list, update};
use crate::AppState;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/grades", http_post(create::<Grade>).get(list::<Grade>))
        .route("/grades/:id", http_patch(update::<Grade>))
        .route("/grades/:id", http_delete(delete::<Grade>))
        .with_state(state)
}
