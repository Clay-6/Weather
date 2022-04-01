#![allow(non_snake_case)]

use serde_json::Value;
use std::env;
use std::io::{self, Write};

static BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("WEATHER_API_KEY").unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let location = &args[1];
        ShowData(&api_key, location).await?;

        std::process::exit(0);
    }

    loop {
        let mut input = String::new();
        print!("Enter a place name or type 'quit' to exit: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let location = match input.trim().to_lowercase().as_str() {
            "quit" => break,
            s => String::from(s),
        };

        ShowData(&api_key, &location).await?;
    }

    Ok(())
}

async fn ShowData(api_key: &String, location: &String) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
