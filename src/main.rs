mod params;

extern crate structopt;
use params::Params;
use structopt::StructOpt;

use std::fs::File;
use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let params = Params::from_args();
    println!("{:?}", &params);

    let mut input: Box<dyn Read> = match params.input_file {
        Some(ref file) => Box::new(File::open(file)?),
        None => Box::new(io::stdin()),
    };

    let mut output: Box<dyn Write> = match params.output_file {
        Some(ref file) => Box::new(File::create(file)?),
        None => Box::new(io::stdout()),
    };
    output.write_all(b"Test output\n").unwrap();

    Ok(())
}
