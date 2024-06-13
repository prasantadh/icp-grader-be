use axum::extract::{Path, State};
use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::{Json, Router};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};

use crate::schema::{create, delete, list, update};
use crate::schema::{Role, User, ValidatedCollection};
use crate::{AppState, Error, Result};

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
    Json(item): Json<StudentForCU>,
) -> Result<Json<InsertOneResult>> {
    let teacher = User::new_student(&item.name, &item.email, &item.campus_id);
    let result = state
        .db
        .collection::<User>(User::name())
        .insert_one(teacher, None)
        .await?;
    Ok(Json(result))
}

pub async fn list_students(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let mut cursor = state
        .db
        .collection::<User>(User::name())
        .find(doc! {"role": "Student"}, None)
        .await?;
    let mut result: Vec<User> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    Ok(Json(result))
}

pub async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(update): Json<StudentForCU>,
) -> Result<Json<UpdateResult>> {
    let update = to_document(&update).map_err(|_| Error::MiscError)?;
    let result = state
        .db
        .collection::<User>(User::name())
        .update_one(doc! {"_id": id}, doc! {"$set": update}, None)
        .await?;
    Ok(Json(result))
}

pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
) -> Result<Json<DeleteResult>> {
    let result = state
        .db
        .collection::<User>(User::name())
        .delete_one(doc! {"_id": id}, None)
        .await?;
    Ok(Json(result))
}
