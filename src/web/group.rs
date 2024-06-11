use std::result;

use axum::extract::{Path, State};
use axum::routing::{delete, patch};
use axum::{routing::post, Json, Router};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::schema::{Group, ValidatedCollection};
use crate::{AppState, Result};

pub fn routes(state: crate::AppState) -> Router {
    Router::new()
        .route("/groups", post(create_group).get(list_groups))
        .route("/groups/:id", patch(update_group))
        .route("/groups/:id", delete(delete_group))
        .route("/groups/test/:id", patch(update::<Group>))
        .with_state(state)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupForCreate {
    pub name: String,
    pub year: u32,
}

pub async fn create_group(
    State(state): State<AppState>,
    Json(g): Json<GroupForCreate>,
) -> Result<Json<InsertOneResult>> {
    let result = state
        .db
        .collection::<GroupForCreate>(Group::name())
        .insert_one(g.clone(), None)
        .await?;
    Ok(Json(result))
}

pub async fn list_groups(State(state): State<AppState>) -> Result<Json<Vec<Group>>> {
    let mut cursor = state
        .db
        .collection::<Group>(Group::name())
        .find(doc! {}, None)
        .await?;

    let mut result: Vec<Group> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    Ok(Json(result))
}

pub async fn update<C>(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(j): Json<C>,
) -> Result<Json<UpdateResult>>
where
    C: ValidatedCollection,
{
    let result = state
        .db
        .collection::<C>(C::name())
        .update_one(doc! {"_id": id}, doc! {}, None)
        .await?;
    Ok(Json(result))
}

pub async fn update_group(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(g): Json<GroupForCreate>,
) -> Result<()> {
    let result = state
        .db
        .collection::<Group>(Group::name())
        .update_one(
            doc! {"_id": id},
            doc! {
                "_id": id,
                "name": g.name,
                "year": g.year,
            },
            None,
        )
        .await?;
    Ok(())
}

pub async fn delete_group(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
) -> Result<Json<DeleteResult>> {
    let result = state
        .db
        .collection::<Group>(Group::name())
        .delete_one(doc! {"_id": id}, None)
        .await?;
    Ok(Json(result))
}
