mod user;
pub use user::{NewUser, Role, User};

mod section;
pub use section::{Level, Section, Year};

// should there be a default admin we make here?
// or perhaps it should go into migrations
