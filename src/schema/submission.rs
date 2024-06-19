use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

use super::Question;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade {
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub student_id: ObjectId,
    pub assessment_id: ObjectId,
    pub repo: String,
    pub grade: Option<Grade>,
}

impl Submission {
    pub fn new(student_id: ObjectId, assessment_id: ObjectId, repo: String) -> Self {
        Self {
            id: None,
            student_id,
            assessment_id,
            repo,
            grade: None,
        }
    }
}

impl ValidatedCollection for Submission {
    fn name() -> &'static str {
        "submissions"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
