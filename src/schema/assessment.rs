use mongodb::bson::{doc, oid::ObjectId};

use crate::schema::ValidatedCollection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reduction {
    pub reason: String,
    pub marks: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub full_marks: u32,
    pub reductions: Vec<Reduction>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub name: String,
    pub subject_id: ObjectId,
    pub questions: Vec<Question>,
}

impl Assessment {
    pub fn new(name: String, subject_id: ObjectId, questions: Vec<Question>) -> Self {
        Self {
            id: None,
            name,
            subject_id,
            questions,
        }
    }
}

impl ValidatedCollection for Assessment {
    fn name() -> &'static str {
        "assessments"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
