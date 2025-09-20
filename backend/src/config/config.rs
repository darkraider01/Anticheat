use serde::Deserialize;
use config as config_crate;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub application: Application,
}

pub fn config() -> Result<Configuration, config_crate::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let configuration_directory = base_path.join("configuration");

    let mut settings = config_crate::Config::new();
    settings
        .merge(config_crate::File::from(configuration_directory.join("base")).required(true))?
        .merge(config_crate::Environment::with_prefix("APP").separator("__"))?;

    settings.try_into()
}