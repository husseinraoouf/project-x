use std::{env, fs, io::Error};

pub mod structs;
use once_cell::sync::Lazy;
pub use structs::Config;

static DEFAULT_CONFIG_FILE: &str = "config/config.hjson";

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::init().expect("Failed to load settings file"));

impl Config {
    /// Reads config from configuration file.
    ///
    /// Warning: Only call this once.
    pub fn init() -> Result<Self, ()> {
        // Read the config file
        let config = deser_hjson::from_str::<Config>(&Self::read_config_file().unwrap()).unwrap();

        Ok(config)
    }

    pub fn get_database_url(&self) -> String {
        let conf = &self.database;
        format!(
            "postgres://{}:{}@{}:{}/{}",
            conf.user, conf.password, conf.host, conf.port, conf.database,
        )
    }

    pub fn get_config_location() -> String {
        env::var("CONFIG_LOCATION").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string())
    }

    pub fn read_config_file() -> Result<String, Error> {
        fs::read_to_string(Self::get_config_location())
    }
}
