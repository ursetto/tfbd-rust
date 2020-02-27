use structopt::StructOpt;

#[derive(StructOpt,Debug)]
enum Cli {
    Decode {
        filename: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Decode { filename } => { println!("decode filename: {:?}", filename); }
    }
}
