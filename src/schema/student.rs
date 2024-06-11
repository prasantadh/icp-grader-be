use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub email: String,
    pub group_id: ObjectId,
}

impl ValidatedCollection for Student {
    fn name() -> &'static str {
        "students"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
