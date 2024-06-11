use mongodb::bson::doc;

use crate::schema::ValidatedCollection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment;

impl ValidatedCollection for Assessment {
    fn name() -> &'static str {
        "assessments"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
