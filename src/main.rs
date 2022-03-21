use serde_json::Value;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("WEATHER_API_KEY").unwrap();
    const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let location = &args[1];
        let request_url = format!("{}?appid={}&q={}&units=metric", BASE_URL, api_key, location);
        let response = reqwest::get(request_url).await?.text().await?;

        let data: Value = serde_json::from_str(response.as_str())?;
        let weather = &data["weather"][0]["description"];
        let temperature = &data["main"]["temp"];

        if weather.is_null() || temperature.is_null() {
            eprintln!("Data for \"{}\" not found", location);
        }

        println!(
            "Weather description: {}\n\
        Temperature: {}⁰C\n",
            weather, temperature
        );
    }

    loop {
        let mut input = String::new();
        print!("Enter a place name or type 'quit' to exit: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let location = input.trim().to_lowercase();
        if location == String::from("quit") {
            break;
        }

        let request_url = format!("{}?appid={}&q={}&units=metric", BASE_URL, api_key, location);
        let response = reqwest::get(request_url).await?.text().await?;

        let data: Value = serde_json::from_str(response.as_str())?;
        let weather = &data["weather"][0]["description"];
        let temperature = &data["main"]["temp"];

        if weather.is_null() || temperature.is_null() {
            eprintln!("Data for \"{}\" not found\n", location);
            continue;
        }

        println!(
            "Weather description: {}\n\
            Temperature: {}⁰C\n",
            weather, temperature
        );
    }
    Ok(())
}
