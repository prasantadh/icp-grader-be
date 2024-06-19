use std::collections::vec_deque;
use std::ops::Sub;
use std::result;

use crate::schema::{self, create, delete, list, update, Question, Role, Subject, Submission};
use crate::schema::{Assessment, ValidatedCollection};
use crate::web::assessment;
use crate::{AppState, Context, Error, Result};

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

use super::subject;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/assessments",
            http_post(create_assessment).get(list_assessments),
        )
        .route("/assessments/:id", http_patch(update_assessment))
        .route("/assessments/:id", http_delete(delete_assessment))
        .route(
            "/assessments/:id/submissions",
            http_get(assessment_submissions),
        )
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssessmentForCreate {
    subject_id: ObjectId,
    name: String,
    questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssessmentForUpdate {
    name: String,
    questions: Vec<Question>,
}

pub async fn create_assessment(
    State(state): State<AppState>,
    context: Context,
    Json(assessment): Json<AssessmentForCreate>,
) -> Result<Json<InsertOneResult>> {
    if context.role() == Role::Student {
        return Err(Error::UnauthorizedActionError);
    }
    // check it is a valid subject id
    let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
    // check the current user is either teaches that class or is admin
    if !subject.members.contains(&context.user_id()) && context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let assessment = Assessment::new(assessment.name, assessment.subject_id, assessment.questions);
    let result = schema::create(&state.db, assessment).await?;
    Ok(Json(result))
}

pub async fn get_assessments(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<Assessment>> {
    let mut assessment = schema::get::<Assessment>(&state.db, id).await?;
    let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
    let assessment = match context.role() {
        Role::Student => {
            // verify the student is in the subject for that assignment
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
            assessment.questions = vec![];
            assessment
        }
        Role::Teacher => {
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
            assessment
        }
        Role::Admin => assessment,
    };
    Ok(Json(assessment))
}

pub async fn list_assessments(
    State(state): State<AppState>,
    context: Context,
) -> Result<Json<Vec<Assessment>>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let result = schema::list::<Assessment>(&state.db, doc! {}).await?;
    Ok(Json(result))
}

pub async fn update_assessment(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(update): Json<AssessmentForUpdate>,
) -> Result<Json<UpdateResult>> {
    // FIXME should we update assessments if there are already grades on them?
    let mut assessment = schema::get::<Assessment>(&state.db, id).await?;
    let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
    match context.role() {
        Role::Student => return Err(Error::UnauthorizedActionError),
        Role::Teacher => {
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
        Role::Admin => (),
    };
    let update = to_document(&update).map_err(|_| Error::MongoSerializationError)?;
    let result = schema::update::<Assessment>(&state.db, id, doc! {"$set": update}).await?;
    Ok(Json(result))
}

pub async fn delete_assessment(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<DeleteResult>> {
    // FIXME will likely need to delete associated submissions and/or grades?
    let mut assessment = schema::get::<Assessment>(&state.db, id).await?;
    let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
    match context.role() {
        Role::Student => return Err(Error::UnauthorizedActionError),
        Role::Teacher => {
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
        Role::Admin => (),
    };
    let result = schema::delete::<Assessment>(&state.db, id).await?;
    Ok(Json(result))
}

pub async fn assessment_submissions(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<Vec<Submission>>> {
    // students are unauthorized,
    // teachers can see all submissions for their subjects
    // admin can see all submissions for everything
    let assessment = schema::get::<Assessment>(&state.db, id).await?;
    match context.role() {
        Role::Student => return Err(Error::UnauthorizedActionError),
        Role::Admin => (),
        Role::Teacher => {
            // TODO this is probably best refactored out to a function
            // only submissions for the subjects they teach
            let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
    }
    let result = schema::list::<Submission>(&state.db, doc! {"assessment_id": id}).await?;
    Ok(Json(result))
}
