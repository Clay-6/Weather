use serde::{Deserialize, Serialize};
use weather_rs::Unit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub default_units: Unit,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: std::env::var("WEATHER_API_KEY").unwrap(),
            default_units: Unit::Standard,
        }
    }
}
