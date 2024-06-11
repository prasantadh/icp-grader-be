use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    year: u32,
}

impl ValidatedCollection for Group {
    fn name() -> &'static str {
        "groups"
    }

    fn validator() -> Document {
        // TODO now implement what the validation document should look like for this
        doc! {}
    }
}
