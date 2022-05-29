use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    pub location: Option<String>,
}
