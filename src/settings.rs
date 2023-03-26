use serde_derive::Deserialize;
use config::{Config, Environment};
use dotenv::dotenv;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Settings {
    pub(crate) db_conn: String,
    pub(crate) sentry_dsn: String,
}

pub fn load() -> Result<Settings, config::ConfigError> {
    // Load the .env file
    dotenv().ok();

    println!("{:?}", std::env::var("APP_DB_CONN"));

    let config = Config::builder()
        .add_source(
            Environment::with_prefix("APP")
                .try_parsing(true)
        ).build().unwrap();


    // Deserialize the environment variables into the Settings struct
    config.try_deserialize()
}

