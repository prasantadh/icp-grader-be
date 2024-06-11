use mongodb::bson::doc;

use crate::schema::ValidatedCollection;

pub struct Student;

impl ValidatedCollection for Student {
    fn name() -> &'static str {
        "students"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
