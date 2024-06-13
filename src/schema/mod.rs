mod assessment;
mod subject;
mod submission;
mod user;
mod utils;

pub use crate::{Error, Result};

pub use assessment::Assessment;
pub use subject::Semester;
pub use subject::Subject;
pub use submission::Submission;
pub use user::{Role, User};
pub use utils::{create, delete, list, update};

use mongodb::{bson::Document, options::CreateCollectionOptions, Database};

pub async fn init(db: &Database) -> Result<()> {
    // TODO: for dev only, fix this later
    // mostly this is so that schema validation can be used
    db.drop(None).await?;
    init_collection::<Subject>(db).await?;
    init_collection::<User>(db).await?;
    init_collection::<Assessment>(db).await?;
    init_collection::<Submission>(db).await?;
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
