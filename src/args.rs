use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub server: PathBuf,
    #[arg(short, long)]
    pub out: PathBuf,
    #[arg(short, long)]
    pub include: PathBuf,
}
