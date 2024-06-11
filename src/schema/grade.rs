use mongodb::bson::doc;

use crate::schema::ValidatedCollection;

pub struct Grade;

impl ValidatedCollection for Grade {
    fn name() -> &'static str {
        "grades"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
