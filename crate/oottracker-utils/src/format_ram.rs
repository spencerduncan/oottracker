#![deny(
    rust_2018_idioms,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    warnings
)]
#![forbid(unsafe_code)]

use {
    oottracker::ram::{self, Ram},
    std::{
        fs::File,
        io::{self, prelude::*},
        path::PathBuf,
    },
    thiserror::Error,
};

#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Decode(#[from] ram::DecodeError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[wheel::main]
fn main(args: Args) -> Result<(), Error> {
    let mut buf = Vec::with_capacity(ram::SIZE);
    File::open(args.input)?.read_to_end(&mut buf)?;
    println!("{:#?}", Ram::from_bytes(&buf)?);
    Ok(())
}
