use structopt::StructOpt;
use std::path::PathBuf;
use std::io;
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

fn decode(filename: PathBuf) -> io::Result<()> {
    let mut f = File::open(&filename)?;
    println!("decode filename: {}", filename.display());
    let record_count = f.read_u16::<LE>()?;
    println!("# TFBD ({} records total)", record_count);
    Ok(())
}
