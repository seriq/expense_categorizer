use crate::categorizer::*;
use crate::model::*;
use crate::parser::*;
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules_from_file().expect("Error parsing rules.");
    let bookings = parse_bookings_from_file().expect("Error parsing bookings.");
    let mut categorized_bookings: Vec<CategorizedBooking> = Vec::new();
    for booking in bookings {
        categorized_bookings.push(categorize_booking(booking, &rules));
    }
    for booking in categorized_bookings {
        match booking.category.as_str() {
            "Warmmiete" => println!("Warmmiete: {:?}", booking),
            "Internet" => println!("Internet: {:?}", booking),
            "Strom" => println!("Strom: {:?}", booking),
            _ => (),
        }
    }
    Ok(())
}
