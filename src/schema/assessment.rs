use mongodb::bson::doc;

use crate::schema::ValidatedCollection;

pub struct Assessment;

impl ValidatedCollection for Assessment {
    fn name() -> &'static str {
        "assessments"
    }

    fn validator() -> mongodb::bson::Document {
        doc! {}
    }
}
