use std::result;

use axum::extract::{Path, State};
use axum::routing::{
    delete as http_delete, get as http_get, patch as http_patch, post as http_post,
};
use axum::{middleware, Json, Router};
use axum_extra::routing::RouterExt;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, to_document, Bson, Document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};

use crate::schema::{
    self, create, delete, list, update, Assessment, Role, User, ValidatedCollection,
};
use crate::schema::{Semester, Subject};
use crate::AppState;
use crate::Context;
use crate::{Error, Result};

use super::teacher::TeacherForCU;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/subjects", http_get(list_subjects))
        .route("/subjects/:id", http_patch(update_subject))
        .route("/subjects/:id", http_delete(delete_subject))
        .route("/subjects/:id/members", http_post(join_subject))
        .route("/subjects/:id/members", http_delete(drop_subject))
        .route("/subjects/:id/assessments", http_get(subject_assessments))
        // .route_layer(middleware::from_fn(crate::web::middleware::require_auth))
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
    Json(subject): Json<SubjectForCU>,
    context: Context,
) -> Result<Json<InsertOneResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let subject = Subject::new(&subject.name, subject.year, subject.semester);
    let result = schema::create(&state.db, subject).await?;
    Ok(Json(result))
}

pub async fn list_subjects(
    State(state): State<AppState>,
    context: Context,
) -> Result<Json<Vec<Subject>>> {
    let filter = match context.role() {
        Role::Admin => doc! {},
        _ => doc! {"members" : context.user_id() },
    };
    let result = schema::list(&state.db, filter).await?;
    Ok(Json(result))
}

pub async fn update_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(update): Json<SubjectForCU>,
) -> Result<Json<UpdateResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let update = to_document(&update).map_err(|_| Error::MongoSerializationError)?;
    let result = schema::update::<Subject>(&state.db, id, doc! { "$set" : update}).await?;
    Ok(Json(result))
}

pub async fn delete_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<DeleteResult>> {
    // FIXME will likely need to delete associated assessments and submissions
    // while dropping a subject
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let result = schema::delete::<Subject>(&state.db, id).await?;
    Ok(Json(result))
}

pub async fn join_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(user_id): Json<ObjectId>,
) -> Result<Json<UpdateResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }

    let user = schema::get::<User>(&state.db, user_id).await?;
    let result =
        schema::update::<Subject>(&state.db, id, doc! {"$addToSet": doc! {"members": user_id}})
            .await?;

    let result =
        schema::update::<User>(&state.db, user_id, doc! {"$addToSet": doc! {"courses": id}})
            .await?;

    Ok(Json(result))
}

pub async fn drop_subject(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(user_id): Json<ObjectId>,
) -> Result<Json<UpdateResult>> {
    // FIXME may be we need to create a transaction where there are more than
    // one queries running?
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let user = schema::get::<User>(&state.db, user_id).await?;

    let result =
        schema::update::<Subject>(&state.db, id, doc! {"$pull": doc!{"members": user_id}}).await?;

    // FIXME should probably also pull submissions/grades/assignments for this subject
    let result =
        schema::update::<User>(&state.db, user_id, doc! {"$pull": doc!{"courses": id}}).await?;

    Ok(Json(result))
}

pub async fn subject_assessments(
    State(state): State<AppState>,
    context: Context,
    Path(id): Path<ObjectId>,
) -> Result<Json<Vec<Assessment>>> {
    let subject = schema::get::<Subject>(&state.db, id).await?;
    // students can only access for subjects they are enrolled in
    // techers can only access for subjects they teach
    // admin can access all
    match context.role() {
        Role::Admin => (),
        // FIXME in the future if parents are added they should be handled separately maybe?
        _ => {
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
    };
    let result = schema::list::<Assessment>(&state.db, doc! { "subject_id": id}).await?;
    Ok(Json(result))
}
