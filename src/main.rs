use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
enum Cli {
    Decode { filename: PathBuf }
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Decode { filename } => { decode(filename); }
    }
}

fn decode(filename: PathBuf) {
    println!("decode filename: {}", filename.to_str().unwrap());
}
