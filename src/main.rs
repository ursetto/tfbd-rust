use structopt::StructOpt;
use std::path::PathBuf;
use std::io;
use std::fs::File;
use byteorder::{LE, ReadBytesExt};
use anyhow::{ensure, Result};

#[derive(StructOpt)]
#[structopt(global_settings = &[structopt::clap::AppSettings::ColoredHelp])]
enum Cli {
    Decode { filename: PathBuf }
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    match args {
        Cli::Decode { filename } => { decode_file(filename) }
    }
}

fn decode_file(filename: PathBuf) -> Result<()> {
    let f = File::open(&filename)?;
    let mut r = io::BufReader::new(f);
    // println!("Decoding filename: {}", filename.display());
    decode(&mut r)?;
    Ok(())
}

fn decode(r: &mut impl io::Read) -> Result<()> {
    let record_count = r.read_u16::<LE>()?;
    println!("# TFBD ({} records total)", record_count);
    decode_2x(r)?;
    decode_4x(r)?;
    decode_6x(r)?;
    Ok(())
}

fn decode_2x(r: &mut impl io::Read) -> Result<()> {
    let section_count = r.read_u16::<LE>()?;
    println!("# 2x section ({} records)", section_count);
    for _ in 0..section_count {
        let rtype = r.read_u8()?;
        ensure!(rtype & 0xf0 == 0x20, "expected 2x section, got {:02X}", rtype);
        let var_len = r.read_u8()?;
        let offset = r.read_u32::<LE>()?;
        let area_len = r.read_u16::<LE>()?;
        assert_eq!(var_len, 0, "2x section var_len must be 0");
        match rtype {
            // An enum would work but how useful is not clear.
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

fn decode_4x(r: &mut impl io::Read) -> io::Result<()> {
    let section_count = r.read_u16::<LE>()?;
    println!("# 4x section ({} records)", section_count);
    for _ in 0..section_count {
        let rtype   = r.read_u8()?;
        assert!(rtype & 0xf0 == 0x40, "expected 4x section, got {:02X}", rtype);
        let var_len = r.read_u8()?;
        assert_ne!(var_len, 0); // all types require a p-string
        let address = r.read_u32::<LE>()?;
        let count   = r.read_u16::<LE>()?;
        let var_str = read_pascal_string(r, var_len)?;

        match rtype {
            0x40 => println!("LAB +${:04X}, {}         # {:04X}",
                             address, var_str, count),
            0x44 => {
                assert_eq!(count, 1);
                println!("EQU  ${:04X}, {}", address, var_str);
            },
            _ => println!("rtype {:02X} var_len {:02X} address {:08X} count {:04X} {}",
                          rtype, var_len, address, count, var_str),
        }
    }
    Ok(())
}

fn decode_6x(r: &mut impl io::Read) -> io::Result<()> {
    let section_count = r.read_u16::<LE>()?;
    println!("# 6x section ({} records)", section_count);
    for _ in 0..section_count {
        let rtype   = r.read_u8()?;
        assert!(rtype & 0xf0 == 0x60, "expected 6x section, got {:02X}", rtype);
        let var_len = r.read_u8()?;
        let offset  = r.read_u32::<LE>()?;
        let count   = r.read_u32::<LE>()?;
        let arg     = r.read_u32::<LE>()?;
        let var_str = read_pascal_string(r, var_len)?;

        match rtype {
            0x60 => {
                assert_eq!(var_len, 0);
                println!("ORG +${:04X}, ${:04X}, L${:04X}",
                         offset, arg, count);
            },
            0x61 => {
                assert_eq!(count, 1);
                assert_eq!(var_len, 0);
                println!("MX  +${:04X}, %{:02X}", offset, arg);
            },
            0x66 => {
                assert_eq!(count, 1);
                println!("COM +${:04X}, {}", offset, var_str);
            },
            _ => println!("{:02X} {:02X} {:08X} {:08X} {:08X} {}",
                          rtype, var_len, offset, count, arg, var_str),
        }
    }
    Ok(())
}

fn apple_to_ascii(data: &[u8]) -> String {
    let ascii_data: Vec<u8> = data.iter().map(|x| x & 0x7f).collect();
    // Because we strip bit 7, str::from_utf8_unchecked would also work.
    String::from_utf8(ascii_data)
        .expect("invalid utf8 in pascal string")
}

// Read a pascal string from r, expecting it to be of length len.
fn read_pascal_string(r: &mut impl io::Read, len: u8) -> io::Result<String> {
    // Note: this could be an Option<String> for better type-checking for
    // types without a string. An empty string is really a p-string
    // with a 0 length byte, whereas a missing p-string is not present.

    let s = match len {
        0 => "".to_string(),
        _ => {
            assert_eq!(len, r.read_u8()?);
            let mut var_data = vec![0; len as usize];
            r.read_exact(&mut var_data)?;
            apple_to_ascii(&var_data)
        }
    };
    Ok(s)
}
