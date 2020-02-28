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
        Cli::Decode { filename } => { decode_file(filename).unwrap(); }
    }
}

fn decode_file(filename: PathBuf) -> io::Result<()> {
    let f = File::open(&filename)?;
    let r = io::BufReader::new(f);
    println!("Decoding filename: {}", filename.display());
    decode(r)?;
    Ok(())
}

fn decode(mut r: impl io::Read) -> io::Result<()> {
    let record_count = r.read_u16::<LE>()?;
    println!("# TFBD ({} records total)", record_count);
    decode_2x(r)?;
    Ok(())
}

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
