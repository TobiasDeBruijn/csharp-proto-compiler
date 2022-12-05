use crate::args::Args;
use crate::proto_file::get_proto_files;
use crate::protoc::compile_proto;
use clap::Parser;
use color_eyre::eyre::Result;

mod args;
mod proto_file;
mod protoc;

fn main() -> Result<()> {
    setup_logging()?;
    let args = Args::parse();

    let protos = get_proto_files(&args.server)?;
    for proto in protos {
        compile_proto(&proto, &args.include, &args.server, &args.out)?;
    }

    Ok(())
}

fn setup_logging() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().compact().init();

    Ok(())
}
