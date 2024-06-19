use std::result;

use axum::extract::{Path, State};
use axum::routing::{
    delete as http_delete, get as http_get, patch as http_patch, post as http_post,
};
use axum::{Json, Router};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};

use crate::schema;
use crate::schema::{Role, User, ValidatedCollection};
use crate::{AppState, Context, Error, Result};

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/students", http_post(create_student).get(list_students))
        .route("/students/:id", http_patch(update_student))
        .route("/students/:id", http_delete(delete_student))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentForCU {
    name: String,
    email: String,
    campus_id: String,
}

pub async fn create_student(
    State(state): State<AppState>,
    context: Context,
    Json(student): Json<StudentForCU>,
) -> Result<Json<InsertOneResult>> {
    // only admin can create students
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let user = User::new_student(&student.name, &student.email, &student.campus_id);
    let result = schema::create(&state.db, user).await?;
    Ok(Json(result))
}

pub async fn list_students(
    State(state): State<AppState>,
    context: Context,
) -> Result<Json<Vec<User>>> {
    let filter = match context.role() {
        Role::Student => doc! { "_id": context.user_id() },
        // TODO teachers can only see the students they teach
        Role::Teacher => doc! {},
        Role::Admin => doc! { "role": "student"},
    };
    let result = schema::list(&state.db, filter).await?;
    Ok(Json(result))
}

pub async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(update): Json<StudentForCU>,
) -> Result<Json<UpdateResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let update = doc! { "$set": to_document(&update).map_err(|e| Error::MongoSerializationError)?};
    let result = schema::update::<User>(&state.db, id, update).await?;
    Ok(Json(result))
}

pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<DeleteResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let result = schema::delete::<User>(&state.db, id).await?;
    Ok(Json(result))
}
