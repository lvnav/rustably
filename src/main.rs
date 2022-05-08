use std::env;
use std::error::Error;
use std::path::PathBuf;

mod lib;
use lib::parser::Parser as Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = Parser;

    parser.handle(get_filepath());

    Ok(())
}

fn get_filepath() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    let mut filepath = PathBuf::new();
    filepath.push(args[1].as_str());

    filepath
}

