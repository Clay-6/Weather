mod cli;
mod config;
mod utils;

use anyhow::Result;
use clap::Parser as _;
use config::Config;
use std::io::{self, Write};

use cli::Args;

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = confy::load("Weather-Rs")?;
    let args = Args::parse();

    if let Some(location) = args.location {
        let (temp, desc) = utils::get_data(BASE_URL, &config.api_key(), &location).await?;
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

        let (temp, desc) = utils::get_data(BASE_URL, &config.api_key(), &location).await?;
        println!("Temperature: {temp}\nDescription: {desc}");
    }

    Ok(())
}
