use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub api_key_prefix: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir()
        .map_err(|e| ConfigError::Message(format!("Failed to determine current directory: {}", e)))?;
    let configuration_directory = base_path.join("configuration");

    // Updated to use new config crate API
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.toml")).required(false))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    settings.try_deserialize()
}
