use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::schema::ValidatedCollection;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
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
    // TODO these probably do not need to be public. See where I am using them
    pub role: Role,
    pub courses: Vec<ObjectId>,
}

impl User {
    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }

    fn new(name: &String, email: &String, campus_id: Option<String>, role: Role) -> Self {
        User {
            id: None,
            campus_id,
            name: name.clone(),
            email: email.clone(),
            role,
            courses: vec![],
        }
    }

    pub fn new_teacher(name: &String, email: &String) -> Self {
        Self::new(name, email, None, Role::Teacher)
    }

    pub fn new_student(name: &String, email: &String, campus_id: &String) -> Self {
        Self::new(name, email, Some(campus_id.clone()), Role::Student)
    }

    pub fn new_admin(name: &String, email: &String) -> Self {
        Self::new(name, email, None, Role::Admin)
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
