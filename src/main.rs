use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod lib;
use lib::parser::Parser as Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = Parser;

    let assembled_program = parser.handle(get_filepath());

    let mut output = File::create("foo.hack").expect("Error during dist file creation");
    write!(output, "{}", assembled_program).expect("Error during write into dist file");

    Ok(())
}

fn get_filepath() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    let mut filepath = PathBuf::new();
    filepath.push(args[1].as_str());

    filepath
}

