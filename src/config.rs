use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
}

impl Config {
    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: std::env::var("WEATHER_API_KEY").unwrap(),
        }
    }
}
