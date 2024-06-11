use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade;

impl ValidatedCollection for Grade {
    fn name() -> &'static str {
        "grades"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
