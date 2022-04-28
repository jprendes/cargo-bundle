use std::path::PathBuf;
use clap::Parser;

pub mod bundle;
pub mod error;

/// Bundle the content of a crate into a single rust source file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the entry file
    entry: PathBuf,
}

fn parse_args() -> Args {
    Args::parse_from(std::env::args().enumerate().filter_map(|(i, arg)| {
        match (i, arg.as_str()) {
            // Ensure `cargo bundle` works.
            (1, "bundle") => None,
            _ => Some(arg),
        }
    }))
}

fn main() {
    let args = parse_args();
    match bundle::bundle_file(args.entry.as_path()) {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("cargo-bundle error:\n\n{}", error),
    }
}
