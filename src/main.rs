#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    println!("Hello, world!");
    
    Ok(())
}
