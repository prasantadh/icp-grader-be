use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::Router;

use crate::schema::Assessment;
use crate::schema::{create, delete, list, update};
use crate::AppState;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/assessments",
            http_post(create::<Assessment>).get(list::<Assessment>),
        )
        .route("/assessments/:id", http_patch(update::<Assessment>))
        .route("/assessments/:id", http_delete(delete::<Assessment>))
        .with_state(state)
}
