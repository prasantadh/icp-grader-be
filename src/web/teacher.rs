// use std::result;

use axum::extract::{Path, State};
use axum::routing::{delete as http_delete, patch as http_patch, post as http_post};
use axum::{Json, Router};
use diesel::{RunQueryDsl, SelectableHelper};
use futures::TryStreamExt;
// use mongodb::bson::oid::ObjectId;
// use mongodb::bson::{bson, doc, to_document, Document};
// use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::{NewUser, Role, User};
use crate::schema::users;
use crate::AppState;
use crate::Context;
use crate::{Error, Result};

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/teachers", http_post(create_teacher)) // .get(list_teachers))
        // .route("/teachers/:id", http_patch(update_teacher))
        // .route("/teachers/:id", http_delete(delete_teacher))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherForCU {
    name: String,
    email: String,
}

pub async fn create_teacher(
    State(state): State<AppState>,
    context: Context,
    Json(item): Json<TeacherForCU>,
) -> Result<Json<String>> {
    if context.role() != crate::models::Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }

    // TODO: consider calling this struct UserForCreate
    let user = NewUser {
        role: Role::Teacher,
        email: item.email,
        // TODO: should probably also have a name
    };

    use crate::schema::users;

    let connection = state.db_pool.get().await?;
    let result = connection
        .interact(move |c| {
            diesel::insert_into(users::table)
                .values(user)
                .returning(crate::models::User::as_returning())
                .get_result(c)
        })
        .await??;
    Ok(Json(result.email))
}

/*
pub async fn list_teachers(
    State(state): State<AppState>,
    context: Context,
) -> Result<Json<Vec<User>>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let result = schema::list(&state.db, doc! { "role": "teacher"}).await?;
    Ok(Json(result))
}

pub async fn update_teacher(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    context: Context,
    Json(update): Json<TeacherForCU>,
) -> Result<Json<UpdateResult>> {
    if context.role() != Role::Admin {
        return Err(Error::UnauthorizedActionError);
    }
    let update = doc! { "$set" : to_document(&update).map_err(|e| Error::MongoSerializationError)?};
    let result = schema::update::<User>(&state.db, id, update).await?;
    Ok(Json(result))
}

pub async fn delete_teacher(
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
*/
