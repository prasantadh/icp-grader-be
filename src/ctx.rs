use mongodb::bson::oid::ObjectId;

use crate::models;

#[derive(Clone, Debug)]
pub struct Context {
    user_id: i32,
    role: models::Role,
}

// Constructor
impl Context {
    pub fn new(user_id: i32, role: models::Role) -> Self {
        Self { user_id, role }
    }
}

// Property Accessor
impl Context {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn role(&self) -> models::Role {
        self.role
    }
}
