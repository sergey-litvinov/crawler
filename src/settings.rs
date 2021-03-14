use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub telegram: TelegramSettings,
    pub sites: Vec<Site>
}

#[derive(Debug, Deserialize, Clone)]
pub enum ParserType {
    Html,
    Json
}

#[derive(Debug, Deserialize, Clone)]
pub struct Site {
    pub name: String,
    pub parser_type: ParserType,
    pub endpoint: String,
    pub interval_seconds: u64,

    // html settings
    pub selector: Option<String>,
    pub ignored_texts: Option<Vec<String>>,

    // json settings
    pub preflight_request: Option<String>,
    pub csrf_token_selector: Option<String>,
    pub csrf_token_header: Option<String>,
    pub max_allowed_price_uah: Option<f64>
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramSettings {
    pub token: String,
    pub chat_id: String
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default.json"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}.json", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local.json").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app").separator("-"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
