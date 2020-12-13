use crate::categorizer::categorize_booking;
use crate::model::CategorizedBooking;
use crate::parser::{parse_bookings_from_file, parse_rules_from_file};
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules_from_file();
    let bookings = parse_bookings_from_file();
    let categorized_bookings = bookings
        .into_iter()
        .map(|booking| categorize_booking(booking, &rules))
        .collect::<Vec<CategorizedBooking>>();
    for booking in categorized_bookings {
        match booking.category.as_str() {
            "Uncategorized" => println!("Keine Kategorie gefunden: {:?}", booking),
            _ => (),
        }
    }
    Ok(())
}
