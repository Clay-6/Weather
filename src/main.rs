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
    let mut config: Config = confy::load("Weather-Rs")?;
    let args = Args::parse();

    let api_key = args.use_key.unwrap_or(config.api_key);

    if let Some(new_key) = args.set_key {
        config.api_key = new_key;
        confy::store("Weather-Rs", config)?;

        Ok(())
    } else {
        if let Some(location) = args.location {
            let (temp, desc) = utils::get_data(BASE_URL, &api_key, &location, &args.units).await?;
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

            let (temp, desc) = utils::get_data(BASE_URL, &api_key, &location, &args.units).await?;
            println!("Temperature: {temp}\nDescription: {desc}");
        }

        Ok(())
    }
}
