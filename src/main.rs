mod cli;
mod config;

use clap::Parser as _;
use color_eyre::{Report, Result};
use config::Config;
use std::io::{self, Write};

use cli::Args;

const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config = match confy::load("Weather-Rs", None) {
        Ok(cfg) => cfg,
        Err(e) => match e {
            confy::ConfyError::BadTomlData(_) => Config::default(),
            e => return Err(Report::msg(e.to_string())),
        },
    };
    let args = Args::parse();

    let api_key = args.use_key.unwrap_or_else(|| config.api_key.clone());

    if let Some(cmd) = args.command {
        match cmd {
            cli::Cmd::Config {
                api_key,
                default_units,
            } => {
                if let Some(key) = api_key {
                    config.api_key = key
                }
                if let Some(unit) = default_units {
                    config.default_units = unit
                }

                confy::store("Weather-Rs", None, config)?;
                Ok(())
            }
        }
    } else {
        if let Some(location) = args.location {
            let (temp, desc) = weather_rs::get_data(
                BASE_URL,
                &api_key,
                &location,
                &args.units.unwrap_or(config.default_units),
            )
            .await?;
            println!("Temperature: {temp}\nDescription: {desc}");
            std::process::exit(0);
        } else if args.geolocate {
            if let Some(ip) = public_ip::addr().await {
                let city = geolocation::find(&ip.to_string())?
                    .city
                    .chars()
                    .filter(|c| *c != '"')
                    .collect::<String>();
                println!("City detected as {city}");

                let (temp, desc) = weather_rs::get_data(
                    BASE_URL,
                    &api_key,
                    &city,
                    &args.units.unwrap_or(config.default_units),
                )
                .await?;
                println!("Temperature: {temp}\nDescription: {desc}")
            }
        } else {
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

                let (temp, desc) = weather_rs::get_data(
                    BASE_URL,
                    &api_key,
                    &location,
                    &args
                        .units
                        .clone()
                        .unwrap_or_else(|| config.default_units.clone()),
                )
                .await?;
                println!("Temperature: {temp}\nDescription: {desc}");
            }
        }

        Ok(())
    }
}
