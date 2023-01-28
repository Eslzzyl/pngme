mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::*;
use clap::Parser;

extern crate clap;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    match &arguments.command {
        Some(Commands::Encode(params)) => commands::encode(params),
        Some(Commands::Decode(params)) => commands::decode(params),
        Some(Commands::Remove(params)) => commands::remove(params),
        Some(Commands::Print(params)) => commands::print(params),
        None => {}
    }

    Ok(())
}