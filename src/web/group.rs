use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::Group;
use crate::schema::{create, delete, list, update};
use crate::AppState;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/groups", http_post(create::<Group>).get(list::<Group>))
        .route("/groups/:id", http_patch(update::<Group>))
        .route("/groups/:id", http_delete(delete::<Group>))
        .with_state(state)
}
