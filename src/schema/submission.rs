use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

use super::Assessment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: ObjectId,
    pub student_id: ObjectId,
    pub assessment_id: ObjectId,
    pub repo: String,
    pub token: String,
    pub grade: Assessment,
}

impl ValidatedCollection for Submission {
    fn name() -> &'static str {
        "students"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
