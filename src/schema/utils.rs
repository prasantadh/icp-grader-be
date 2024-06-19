use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Database,
};
use serde::{de::DeserializeOwned, Serialize};

use super::ValidatedCollection;
use crate::{Error, Result};

pub async fn create<C>(db: &Database, item: C) -> Result<InsertOneResult>
where
    C: ValidatedCollection + Serialize,
{
    db.collection::<C>(C::name())
        .insert_one(item, None)
        .await
        .map_err(|e| Error::MongoError(e))
}

pub async fn get<C>(db: &Database, id: ObjectId) -> Result<C>
where
    C: ValidatedCollection + DeserializeOwned + Unpin + Send + Sync,
{
    db.collection::<C>(C::name())
        .find_one(doc! {"_id": id}, None)
        .await?
        .ok_or(Error::RecordNotFound)
}

pub async fn list<C>(db: &Database, filter: Document) -> Result<Vec<C>>
where
    C: ValidatedCollection + DeserializeOwned + Unpin + Send + Sync,
{
    let mut cursor = db.collection::<C>(C::name()).find(filter, None).await?;
    let mut result: Vec<C> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }
    Ok(result)
}

pub async fn update<C>(db: &Database, id: ObjectId, updates: Document) -> Result<UpdateResult>
where
    C: ValidatedCollection + Serialize,
{
    db.collection::<C>(C::name())
        .update_one(doc! {"_id": id}, doc! {"$set": updates}, None)
        .await
        .map_err(|e| Error::MongoError(e))
}

pub async fn delete<C>(db: &Database, id: ObjectId) -> Result<DeleteResult>
where
    C: ValidatedCollection,
{
    db.collection::<C>(C::name())
        .delete_one(doc! {"_id": id}, None)
        .await
        .map_err(|e| Error::MongoError(e))
}
