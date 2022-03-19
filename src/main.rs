#![allow(non_snake_case)]
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let API_KEY = env::var("WEATHER_API_KEY").unwrap();
    const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

    loop {
        let mut buffer = String::new();
        print!("Enter a city name or type 'quit' to exit: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");

        let city = buffer.trim().to_lowercase();
        if city == String::from("quit") {
            break;
        }

        let request_url = format!("{}?appid={}&q={}&units=metric", BASE_URL, API_KEY, city);
        let response = reqwest::blocking::get(request_url)?.text()?;
        println!("{:?}\n", response);
    }
    Ok(())
}
