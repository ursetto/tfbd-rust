use structopt::StructOpt;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*; // Read
use std::fs::File;
use byteorder::{LE, ReadBytesExt};

#[derive(StructOpt)]
enum Cli {
    Decode { filename: PathBuf }
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Decode { filename } => { decode(filename).unwrap(); }
    }
}

// TODO: decode should take a writer and have decode_file do the
// file open, for easier testing and composability.
fn decode(filename: PathBuf) -> io::Result<()> {
    let mut f = File::open(&filename)?;
    println!("decode filename: {}", filename.display());
    let record_count = f.read_u16::<LE>()?;
    println!("# TFBD ({} records total)", record_count);
    decode_2x(f)?;
    Ok(())
}

// TODO: Should take a Read trait.
fn decode_2x(mut f: std::fs::File) -> io::Result<()> {
    let section_count = f.read_u16::<LE>()?;
    println!("# 2x section ({} records)", section_count);
    Ok(())
}
