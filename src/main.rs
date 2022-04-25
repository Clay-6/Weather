use anyhow::Result;
use clap::Parser;
use std::env;
use std::io::{self, Write};

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Parser)]
struct Args {
    location: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("WEATHER_API_KEY").unwrap();

    let args = Args::parse();
    if let Some(location) = args.location {
        let (temp, desc) = weather::get_data(BASE_URL, &api_key, &location).await?;
        println!("Temperature: {temp}\nDescription: {desc}");
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

        let (temp, desc) = weather::get_data(BASE_URL, &api_key, &location).await?;
        println!("Temperature: {temp}\nDescription: {desc}");
    }

    Ok(())
}
