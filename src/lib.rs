use serde_json::Value;

pub async fn get_data(
    base_url: &str,
    api_key: &str,
    location: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let request_url = format!("{}?appid={}&q={}&units=metric", base_url, api_key, location);
    let response = reqwest::get(request_url).await?.text().await?;
    let data: Value = serde_json::from_str(response.as_str())?;
    let weather = &data["weather"][0]["description"];
    let temperature = &data["main"]["temp"];
    if weather.is_null() || temperature.is_null() {
        eprintln!("Data for \"{}\" not found", location);
    }

    Ok((temperature.to_string(), weather.to_string()))
}
