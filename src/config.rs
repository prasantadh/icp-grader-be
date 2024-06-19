use std::{env, sync::OnceLock};

use crate::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| 
                //FIXME for some reason, this panic message is not printed, it just exits
                panic!("FATAL - while loading config - Cause: {ex}"))
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
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    // TODO eventually get the name of the env variable that triggered the error
    env::var(name).map_err(|_| Error::ReadEnvError(name))
}
