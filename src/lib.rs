use crate::categorizer::*;
use crate::parser::parse_file;
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let parsed_bookings = parse_file().expect("parse error");
    let mut categorized_bookings: Vec<CategorizedBooking> = Vec::new();
    for booking in parsed_bookings {
        categorized_bookings.push(categorize_booking(booking));
    }
    for booking in categorized_bookings {
        match booking {
            CategorizedBooking::Warmmiete(booking) => println!("{:?}", booking),
            _ => (),
        }
    }
    Ok(())
}
