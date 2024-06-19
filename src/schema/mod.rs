mod assessment;
mod subject;
mod submission;
mod user;
mod utils;

use crate::config;
use crate::web::middleware::Claims;
pub use crate::{Error, Result};

pub use assessment::{Assessment, Question, Reduction};
use jsonwebtoken::encode;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use mongodb::bson::Bson;
pub use subject::Semester;
pub use subject::Subject;
pub use submission::{Grade, Submission};
pub use user::{Role, User};
pub use utils::{create, delete, get, list, update};

use mongodb::{bson::Document, options::CreateCollectionOptions, Database};

async fn create_user(db: &Database, user: User) -> Result<String> {
    let student_id = create::<User>(db, user).await?.inserted_id;
    let id = match student_id {
        Bson::ObjectId(id) => id,
        _ => return Err(Error::UserIdIsNullError),
    };
    let claims = Claims {
        user_id: id,
        // TODO eventually update this expiration to a year's time?
        exp: 2000000000,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config().JWT_SIGNING_SECRET.as_ref()),
    )
    .map_err(|_| Error::AuthError)
}

pub async fn init(db: &Database) -> Result<()> {
    // TODO: for dev only, fix this later
    // mostly this is so that schema validation can be used
    db.drop(None).await?;
    init_collection::<Subject>(db).await?;
    init_collection::<User>(db).await?;
    init_collection::<Assessment>(db).await?;
    init_collection::<Submission>(db).await?;

    // create a student
    let student = User::new_student(
        &"student01".to_string(),
        &"student01@icp.edu.np".to_string(),
        &"ICP01".to_string(),
    );
    let token = create_user(db, student).await?;
    println!("student token: {token}");

    // create a teacher
    let teacher = User::new_teacher(
        &"teacher01".to_string(),
        &"teacher01@icp.edu.np".to_string(),
    );
    let token = create_user(db, teacher).await?;
    println!("teacher token: {token}");

    // create an admin
    let admin = User::new_admin(&"admin".to_string(), &"admin@icp.edu.np".to_string());
    let token = create_user(db, admin).await?;
    println!("admin token: {token}");

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
