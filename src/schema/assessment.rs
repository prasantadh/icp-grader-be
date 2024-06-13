use mongodb::bson::{doc, oid::ObjectId};

use crate::schema::ValidatedCollection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reduction {
    reason: String,
    marks: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    full_marks: u32,
    reductions: Vec<Reduction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment {
    pub id: ObjectId,
    pub name: String,
    pub subject_id: ObjectId,
    pub questions: Vec<Question>,
}

impl ValidatedCollection for Assessment {
    fn name() -> &'static str {
        "assessments"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
