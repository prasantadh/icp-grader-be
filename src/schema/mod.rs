mod assessment;
mod grade;
mod group;
mod student;
mod submission;
mod teacher;
mod utils;

pub use crate::{Error, Result};

pub use assessment::Assessment;
pub use grade::Grade;
pub use group::Group;
pub use student::Student;
pub use teacher::Teacher;
pub use utils::{create, delete, list, update};

use mongodb::{bson::Document, options::CreateCollectionOptions, Database};

pub async fn init(db: &Database) -> Result<()> {
    // TODO: for dev only, fix this later
    db.drop(None).await?;
    init_collection::<Group>(db).await?;
    init_collection::<Teacher>(db).await?;
    init_collection::<Student>(db).await?;
    init_collection::<Assessment>(db).await?;
    init_collection::<Grade>(db).await?;
    Ok(())
}

async fn init_collection<C>(db: &Database) -> Result<()>
where
    C: ValidatedCollection,
{
    let validation_opts = CreateCollectionOptions::builder()
        .validator(C::validator())
        .validation_action(Some(mongodb::options::ValidationAction::Error))
        .validation_level(Some(mongodb::options::ValidationLevel::Moderate))
        .build();
    db.create_collection(C::name(), validation_opts).await?;
    Ok(())
}

pub trait ValidatedCollection {
    fn name() -> &'static str;
    fn validator() -> Document;
}

pub trait ForCreateUpdate {}
