use std::{env, sync::OnceLock};

use crate::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        match Config::load_from_env() {
            Ok(config) => config,
            Err(e) => {
                //FIXME for some reason, this panic message is not printed, it just exits
                panic!("FATAL - while loading config - Cause: {e:?} ");
            }
        }
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub MONGO_CONN_URI: String,
    pub DB_NAME: String,
    pub GOOGLE_OAUTH_CLIENT: String,
    pub GOOGLE_OAUTH_SECRET: String,
    pub GOOGLE_OAUTH_RETURN: String,
    pub JWT_SIGNING_SECRET: String,
    pub ADMIN_EMAIL: String,
    pub GODMODE: bool,
    pub DATABASE_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            MONGO_CONN_URI: get_env("MONGO_CONN_URI")?,
            DB_NAME: get_env("DB_NAME")?,
            GOOGLE_OAUTH_CLIENT: get_env("GOOGLE_OAUTH_CLIENT")?,
            GOOGLE_OAUTH_SECRET: get_env("GOOGLE_OAUTH_SECRET")?,
            GOOGLE_OAUTH_RETURN: get_env("GOOGLE_OAUTH_RETURN")?,
            JWT_SIGNING_SECRET: get_env("JWT_SIGNING_SECRET")?,
            ADMIN_EMAIL: get_env("ADMIN_EMAIL")?,
            DATABASE_URL: get_env("DATABASE_URL")?,
            GODMODE: match get_env("GODMODE")?.as_ref() {
                "true" => true,
                "false" => false,
                _ => return Err(Error::MiscError),
            },
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    // TODO eventually get the name of the env variable that triggered the error
    env::var(name).map_err(|e| Error::ReadEnvError(format!("{name}: {e:?}")))
}
