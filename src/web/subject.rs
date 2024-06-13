use axum::extract::{Path, State};
use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::{Json, Router};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, to_document, Bson, Document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};

use crate::schema::{create, delete, list, update, ValidatedCollection};
use crate::schema::{Semester, Subject};
use crate::AppState;
use crate::{Error, Result};

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/subjects", http_post(create_subject).get(list_subjects))
        .route("/subjects/:id", http_patch(update_subject))
        .route("/subjects/:id", http_delete(delete::<Subject>))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectForCU {
    name: String,
    year: u32,
    semester: Semester,
}

pub async fn create_subject(
    State(state): State<AppState>,
    Json(item): Json<SubjectForCU>,
) -> Result<Json<InsertOneResult>> {
    let subject = Subject::new(&item.name, item.year, item.semester);
    let result = state
        .db
        .collection::<Subject>(Subject::name())
        .insert_one(subject, None)
        .await?;
    Ok(Json(result))
}

pub async fn list_subjects(State(state): State<AppState>) -> Result<Json<Vec<Subject>>> {
    println!("getting results");
    let mut cursor = state
        .db
        .collection::<Subject>(Subject::name())
        .find(doc! {}, None)
        .await
        .unwrap();
    let mut result: Vec<Subject> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    println!("{:?}", result);
    Ok(Json(result))
}

pub async fn update_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(update): Json<SubjectForCU>,
) -> Result<Json<UpdateResult>> {
    let update = to_document(&update).map_err(|_| Error::MiscError)?;
    let result = state
        .db
        .collection::<Subject>(Subject::name())
        .update_one(doc! {"_id": id}, doc! { "$set" : update}, None)
        .await?;
    Ok(Json(result))
}

pub async fn delete_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
) -> Result<Json<DeleteResult>> {
    let result = state
        .db
        .collection::<Subject>(Subject::name())
        .delete_one(doc! {"_id": id}, None)
        .await?;
    Ok(Json(result))
}
