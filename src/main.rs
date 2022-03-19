#![allow(non_snake_case)]
use std::collections::HashMap;
use std::io::{self, Write};

const API_KEY: &str = "a3fa1bfa682d5b4f93efe3abd290edd1";
const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
