use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the input protobuf files
    // TODO rename this parameter
    #[arg(short, long)]
    pub server: PathBuf,
    /// The output directory for the generated C# files
    #[arg(short, long)]
    pub out: PathBuf,
    /// Passed to protoc's `-I` flag
    #[arg(short, long)]
    pub include: PathBuf,
    /// The path to `protoc`.
    /// If not provided, the system's `PATH` will be used to try and find it.
    #[arg(short, long)]
    pub protoc: Option<PathBuf>,
}
