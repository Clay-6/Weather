use clap::{Parser, Subcommand};

use weather_rs::Unit;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The location to see data for
    pub location: Option<String>,
    /// The unit to display temperature in.
    /// Can be `metric`, `imperial`, or `standard`
    #[clap(short, long, default_value_t = Unit::Standard)]
    pub units: Unit,
    /// Override the key set in the configuration
    /// with the one provided
    #[clap(long)]
    pub use_key: Option<String>,
    #[clap(subcommand)]
    pub command: Option<Cmd>,
    /// Get the location using geolocation from your
    /// IP address
    #[clap(short, long)]
    pub geolocate: bool,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Edit the program's configuration values
    Config {
        /// Set an API key to use in requests
        #[clap(long)]
        api_key: Option<String>,
        /// Set the default units used when displaying info
        #[clap(long)]
        default_units: Option<Unit>,
    },
}
