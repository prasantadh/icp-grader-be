use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::{create, delete, list, update};
use crate::AppState;

use crate::schema::Submission;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/submissions",
            http_post(create::<Submission>).get(list::<Submission>),
        )
        .route("/submissions/:id", http_patch(update::<Submission>))
        .route("/submissions/:id", http_delete(delete::<Submission>))
        .with_state(state)
}
