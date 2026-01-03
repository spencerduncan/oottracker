#![deny(
    rust_2018_idioms,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    warnings
)]
#![forbid(unsafe_code)]

#[cfg(windows)]
use {
    oottracker::ram::{self, Ram},
    std::{
        fs::File,
        io::{self, prelude::*},
        path::PathBuf,
    },
    thiserror::Error,
};

#[cfg(windows)]
#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,
}

#[cfg(windows)]
#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Decode(#[from] ram::DecodeError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[cfg(windows)]
#[wheel::main]
fn main(args: Args) -> Result<(), Error> {
    let mut buf = Vec::with_capacity(ram::SIZE);
    File::open(args.input)?.read_to_end(&mut buf)?;
    println!("{:#?}", Ram::from_bytes(&buf)?);
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    eprintln!("oottracker-format-ram is only supported on Windows");
    std::process::exit(1);
}
