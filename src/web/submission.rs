use std::result;

use axum::extract::{Path, State};
use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::{Json, Router};
use axum_extra::routing::RouterExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};

use crate::schema::{
    self, create, delete, list, update, Assessment, Grade, Role, Subject, ValidatedCollection,
};
use crate::web::assessment;
use crate::{AppState, Context, Error, Result};

use crate::schema::Submission;

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route(
            "/submissions",
            http_post(create_submission).get(list_submissions),
        )
        .route("/submissions/:id", http_patch(update_submission))
        .route("/submissions/:id", http_delete(delete_submission))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionForCreate {
    student_id: ObjectId,
    assessment_id: ObjectId,
    repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionForUpdate {
    repo: String,
}

pub async fn create_submission(
    State(state): State<AppState>,
    context: Context,
    Json(submission): Json<SubmissionForCreate>,
) -> Result<Json<InsertOneResult>> {
    let submission = Submission::new(
        submission.student_id,
        submission.assessment_id,
        submission.repo,
    );
    match context.role() {
        Role::Teacher => return Err(Error::UnauthorizedActionError),
        Role::Student => {
            // need to check the student is enrolled in this subject
            if submission.student_id != context.user_id() {
                // cannot submit for other students
                return Err(Error::UnauthorizedActionError);
            }
            let subject = schema::get::<Subject>(&state.db, submission.student_id).await?;
            if !subject.members.contains(&submission.student_id) {
                // cannot submit for un-enrolled subjects
                return Err(Error::UnauthorizedActionError);
            }
        }
        Role::Admin => (),
    };
    // FIXME may be check that the repo link is valid?
    let result = schema::create(&state.db, submission).await?;
    Ok(Json(result))
}

pub async fn get_submission(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<Submission>> {
    let submission = schema::get::<Submission>(&state.db, id).await?;
    match context.role() {
        Role::Student => {
            if context.user_id() != submission.student_id {
                return Err(Error::UnauthorizedActionError);
            }
        }
        Role::Teacher => {
            // only submissions for the subjects they teach
            let assessment = schema::get::<Assessment>(&state.db, submission.assessment_id).await?;
            let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
        Role::Admin => (),
    }
    Ok(Json(submission))
}

pub async fn list_submissions(
    State(state): State<AppState>,
    context: Context,
    Json(submission): Json<Vec<Submission>>,
) -> Result<Json<Vec<Submission>>> {
    if context.role() == Role::Admin {
        return Err(Error::UnauthorizedActionError);
    };
    let result = schema::list::<Submission>(&state.db, doc! {}).await?;
    Ok(Json(result))
}

pub async fn update_submission(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(update): Json<SubmissionForUpdate>,
) -> Result<Json<UpdateResult>> {
    // teacher is not authorized to delete a submission
    // student can only update their own submission
    // admin can update any
    let submission = schema::get::<Submission>(&state.db, id).await?;
    match context.role() {
        Role::Teacher => return Err(Error::UnauthorizedActionError),
        Role::Admin => (),
        Role::Student => {
            if context.user_id() != submission.student_id {
                return Err(Error::UnauthorizedActionError);
            }
        }
    }
    let update = to_document(&update).map_err(|_| Error::MongoSerializationError)?;
    let result = schema::update::<Submission>(&state.db, id, doc! {"set": update}).await?;
    Ok(Json(result))
}

pub async fn delete_submission(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
) -> Result<Json<DeleteResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let result = schema::delete::<Submission>(&state.db, id).await?;
    Ok(Json(result))
}

pub async fn grade_submission(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(grade): Json<Grade>,
) -> Result<Json<UpdateResult>> {
    // students cannot assign themselves grade
    // teachers can assign grades to only students they teach
    // admin can assign to anyone
    let submission = schema::get::<Submission>(&state.db, id).await?;
    match context.role() {
        Role::Student => return Err(Error::UnauthorizedActionError),
        Role::Admin => (),
        Role::Teacher => {
            // TODO this is probably best refactored out to a function
            // only submissions for the subjects they teach
            let assessment = schema::get::<Assessment>(&state.db, submission.assessment_id).await?;
            let subject = schema::get::<Subject>(&state.db, assessment.subject_id).await?;
            if !subject.members.contains(&context.user_id()) {
                return Err(Error::UnauthorizedActionError);
            }
        }
    }
    let update = to_document(&grade).map_err(|_| Error::MongoSerializationError)?;
    let result = schema::update::<Submission>(&state.db, id, doc! {"$set": update}).await?;

    todo!()
}
