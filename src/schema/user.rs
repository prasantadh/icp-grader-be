use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

use super::subject::Subject;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Student,
    Teacher,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    campus_id: Option<String>,
    name: String,
    email: String,
    role: Role,
    courses: Vec<Subject>,
}

impl User {
    pub fn new_teacher(name: &String, email: &String) -> Self {
        User {
            id: None,
            campus_id: None,
            name: name.clone(),
            email: email.clone(),
            role: Role::Teacher,
            courses: vec![],
        }
    }

    pub fn new_student(name: &String, email: &String, campus_id: &String) -> Self {
        User {
            id: None,
            campus_id: Some(campus_id.clone()),
            name: name.clone(),
            email: email.clone(),
            role: Role::Student,
            courses: vec![],
        }
    }
}

impl ValidatedCollection for User {
    fn name() -> &'static str {
        "users"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
