use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: std::env::var("WEATHER_API_KEY").unwrap(),
        }
    }
}
