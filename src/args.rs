use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author="Eslzzyl")]
#[command(version)]
#[command(about="PNGme: A PNG encrypt tool", long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeCommand),
    Decode(DecodeCommand),
    Remove(RemoveCommand),
    Print(PrintCommand)
}

#[derive(Args, Debug)]
pub struct EncodeCommand {
    pub image_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>
}

#[derive(Args, Debug)]
pub struct DecodeCommand {
    pub image_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct RemoveCommand {
    pub image_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct PrintCommand {
    pub image_path: PathBuf
}