use mongodb::bson::doc;

use crate::schema::ValidatedCollection;

pub struct Teacher;

impl ValidatedCollection for Teacher {
    fn name() -> &'static str {
        "teachers"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
