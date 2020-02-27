use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Decode(Decode),
}

#[derive(StructOpt)]
struct Decode {
    filename: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Decode(decode) => { println!("decode filename: {:?}", decode.filename); }
    }
}
