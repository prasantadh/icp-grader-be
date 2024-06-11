use axum::{
    extract::{Path, State},
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use serde::{de::DeserializeOwned, Serialize};

use super::ValidatedCollection;
use crate::{AppState, Result};

pub async fn create<C>(
    State(state): State<AppState>,
    Json(item): Json<C>,
) -> Result<Json<InsertOneResult>>
where
    C: ValidatedCollection + Serialize,
{
    // TODO must assert that id field must be null while create
    // and that the id field stays null while updating
    let result = state
        .db
        .collection::<C>(C::name())
        .insert_one(item, None)
        .await?;
    Ok(Json(result))
}

pub async fn list<C>(State(state): State<AppState>) -> Result<Json<Vec<C>>>
where
    C: ValidatedCollection + DeserializeOwned + Unpin + Send + Sync,
{
    let mut cursor = state
        .db
        .collection::<C>(C::name())
        .find(doc! {}, None)
        .await?;
    let mut result: Vec<C> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    Ok(Json(result))
}

pub async fn update<C>(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(_j): Json<C>,
) -> Result<Json<UpdateResult>>
where
    C: ValidatedCollection,
{
    let result = state
        .db
        .collection::<C>(C::name())
        // TODO need to actually pass in the struct to update
        // but make sure user cannot update the id of the document
        .update_one(doc! {"_id": id}, doc! {}, None)
        .await?;
    Ok(Json(result))
}

pub async fn delete<C>(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
) -> Result<Json<DeleteResult>>
where
    C: ValidatedCollection,
{
    let result = state
        .db
        .collection::<C>(C::name())
        .delete_one(doc! {"_id": id}, None)
        .await?;
    Ok(Json(result))
}
