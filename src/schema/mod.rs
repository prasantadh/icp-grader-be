mod assessment;
mod subject;
mod submission;
mod user;
mod utils;

use crate::config;
use crate::schema;
use crate::web::middleware::Claims;
pub use crate::{Error, Result};

pub use assessment::{Assessment, Question, Reduction};
use jsonwebtoken::encode;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
pub use subject::Semester;
pub use subject::Subject;
pub use submission::{Grade, Submission};
pub use user::{Role, User};
pub use utils::{create, delete, get, list, update};

use mongodb::{bson::Document, options::CreateCollectionOptions, Database};

pub async fn get_token(id: ObjectId) -> Result<String> {
    let claims = Claims {
        user_id: id,
        // TODO eventually update this expiration to a year's time?
        // apparently online recommended is 15 minutes
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
    let id = match schema::create(db, student).await?.inserted_id {
        Bson::ObjectId(id) => id,
        _ => return Err(Error::UserIdIsNullError),
    };
    let token = get_token(id).await?;
    println!("student token: {token}");

    // create a teacher
    let teacher = User::new_teacher(
        &"teacher01".to_string(),
        &"teacher01@icp.edu.np".to_string(),
    );
    let id = match schema::create(db, teacher).await?.inserted_id {
        Bson::ObjectId(id) => id,
        _ => return Err(Error::UserIdIsNullError),
    };
    let token = get_token(id).await?;
    println!("teacher token: {token}");

    // create an admin
    let admin = User::new_admin(&"admin".to_string(), &"admin@icp.edu.np".to_string());
    let id = match schema::create(db, admin).await?.inserted_id {
        Bson::ObjectId(id) => id,
        _ => return Err(Error::UserIdIsNullError),
    };
    let token = get_token(id).await?;
    println!("teacher token: {token}");

    // create developer user for testing
    let developer = User::new_admin(
        &"admin".to_string(),
        &"prasantadhikari1111@gmail.com".to_string(),
    );
    let _ = schema::create(db, developer).await?;
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
