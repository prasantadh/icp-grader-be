use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

use super::{Assessment, User};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Semester {
    Fall,
    Spring,
    Summer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub year: u32,
    pub semester: Semester,
    teachers: Vec<User>,
    students: Vec<User>,
    assessments: Vec<Assessment>,
}

impl Subject {
    pub fn new(name: &String, year: u32, semester: Semester) -> Self {
        Subject {
            id: None,
            name: name.clone(),
            year,
            semester: semester.clone(),
            teachers: vec![],
            students: vec![],
            assessments: vec![],
        }
    }
}

impl ValidatedCollection for Subject {
    fn name() -> &'static str {
        "subjects"
    }

    fn validator() -> Document {
        // TODO now implement what the validation document should look like for this
        doc! {}
    }
}
