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
    // println!("Decoding filename: {}", filename.display());
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
        assert!(rtype & 0xf0 == 0x20, "expected 2x (0x20) section");
        assert_eq!(var_len, 0, "2x section var_len must be 0");
        match rtype {
            0x20 => println!("DB  +${:04X}, ${:02X}", offset, area_len),
            0x21 => println!("DW  +${:04X}, ${:02X}", offset, area_len),
            0x23 => println!("DA  +${:04X}, ${:02X}", offset, area_len),
            0x27 => println!("HEX +${:04X}, ${:02X}", offset, area_len),
            0x28 => println!("DS  +${:04X}, ${:02X}", offset, area_len),
            0x29 => println!("ASC +${:04X}, ${:02X}", offset, area_len),
            _    => println!("rtype {:02X} var_len {:02X} offset {:08X} area_len {:04X}",
                             rtype, var_len, offset, area_len)
        }
    }
    Ok(())
}
