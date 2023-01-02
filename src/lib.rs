use core::fmt;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Unit {
    Standard,
    Metric,
    Imperial,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Standard => write!(f, "standard"),
            Unit::Metric => write!(f, "metric"),
            Unit::Imperial => write!(f, "imperial"),
        }
    }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "metric" | "c" => Ok(Self::Metric),
            "standard" | "k" => Ok(Self::Standard),
            "imperial" | "f" => Ok(Self::Imperial),
            _ => Err(anyhow!("Invalid unit")),
        }
    }
}

/// Retrieves and returns the weather data from the given url using the
/// API key
pub async fn get_data(
    base_url: &str,
    api_key: &str,
    location: &str,
    units: &Unit,
) -> Result<(String, String)> {
    let request_url = format!(
        "{}?appid={}&q={}&units={}",
        base_url, api_key, location, units
    );
    let response = reqwest::get(request_url).await?.text().await?;
    let data: Value = serde_json::from_str(response.as_str())?;
    let weather = &data["weather"][0]["description"];
    let temperature = &data["main"]["temp"];
    if weather.is_null() || temperature.is_null() {
        eprintln!("Data for \"{}\" not found", location);
        Err(anyhow!("Weather data could not be found"))
    } else {
        let temp_suffix = match units {
            Unit::Standard => "⁰K",
            Unit::Metric => "⁰C",
            Unit::Imperial => "⁰F",
        };
        let mut temp = temperature.to_string();
        temp.push_str(temp_suffix);
        Ok((temp, weather.to_string()))
    }
}
