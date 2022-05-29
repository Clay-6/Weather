use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The location to see data for
    pub location: Option<String>,
    /// Override the key set in the configuration
    /// with the one provided
    #[clap(long)]
    pub use_key: Option<String>,
    /// Set a key to use in the config
    #[clap(long)]
    pub set_key: Option<String>,
}
