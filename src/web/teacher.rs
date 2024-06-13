use axum::extract::{Path, State};
use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::{Json, Router};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{bson, doc, to_document, Document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::schema::{create, delete, list, update};
use crate::schema::{User, ValidatedCollection};
use crate::AppState;
use crate::{Error, Result};

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/teachers", http_post(create_teacher).get(list_teachers))
        .route("/teachers/:id", http_patch(update_teacher))
        .route("/teachers/:id", http_delete(delete::<User>))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherForCU {
    name: String,
    email: String,
}

pub async fn create_teacher(
    State(state): State<AppState>,
    Json(item): Json<TeacherForCU>,
) -> Result<Json<InsertOneResult>> {
    let teacher = User::new_teacher(&item.name, &item.email);
    let result = state
        .db
        .collection::<User>(User::name())
        .insert_one(teacher, None)
        .await?;
    Ok(Json(result))
}

pub async fn list_teachers(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let mut cursor = state
        .db
        .collection::<User>(User::name())
        .find(doc! {"role": "Teacher"}, None)
        .await?;
    let mut result: Vec<User> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    Ok(Json(result))
}

pub async fn update_teacher(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(update): Json<TeacherForCU>,
) -> Result<Json<UpdateResult>> {
    let update = to_document(&update).map_err(|_| Error::MiscError)?;
    let result = state
        .db
        .collection::<User>(User::name())
        .update_one(doc! {"_id": id}, doc! {"$set": update }, None)
        .await?;
    Ok(Json(result))
}

pub async fn delete_teacher(
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
