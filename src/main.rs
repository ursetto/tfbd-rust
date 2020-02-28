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
fn decode_2x(mut r: impl io::Read) -> io::Result<()> {
    let section_count = r.read_u16::<LE>()?;
    println!("# 2x section ({} records)", section_count);
    for _ in 0..section_count {
        let rtype = r.read_u8()?;
        let var_len = r.read_u8()?;
        let offset = r.read_u32::<LE>()?;
        let area_len = r.read_u16::<LE>()?;
        println!("rtype {:02x} var_len {:02x} offset {:08x} area_len {:04x}",
                 rtype, var_len, offset, area_len);
    }
    Ok(())
}
