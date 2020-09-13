use crate::parser::parse_file;
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    parse_file().expect("parse error");
    Ok(())
}
