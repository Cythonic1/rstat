use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser,Debug)]
#[command(version, about, long_about = "Improve stat version")]
struct Args {
    #[arg(short, long, help = "file name")]
    file_name: PathBuf
}

fn main() {
    let cli = Args::parse();

    println!("Thank you for the file: {:?}", cli.file_name);

    match std::fs::metadata(cli.file_name){
        Ok(data) =>  {
            println!("{:?}", data)
        },
        Err(err) => {
            eprintln!("Error {err}");
        }

    }
    println!("Hello, world!");
}
