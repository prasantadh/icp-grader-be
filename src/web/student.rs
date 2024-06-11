use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::Student;
use crate::schema::{create, delete, list, update};
use crate::AppState;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/students",
            http_post(create::<Student>).get(list::<Student>),
        )
        .route("/students/:id", http_patch(update::<Student>))
        .route("/students/:id", http_delete(delete::<Student>))
        .with_state(state)
}
