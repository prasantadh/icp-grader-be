use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher;

impl ValidatedCollection for Teacher {
    fn name() -> &'static str {
        "teachers"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
