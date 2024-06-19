use mongodb::bson::oid::ObjectId;

use crate::schema;

#[derive(Clone, Debug)]
pub struct Context {
    user_id: ObjectId,
    role: schema::Role,
}

// Constructor
impl Context {
    pub fn new(user_id: ObjectId, role: schema::Role) -> Self {
        Self { user_id, role }
    }
}

// Property Accessor
impl Context {
    pub fn user_id(&self) -> ObjectId {
        self.user_id
    }

    pub fn role(&self) -> schema::Role {
        self.role
    }
}
