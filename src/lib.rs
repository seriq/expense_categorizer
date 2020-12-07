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
        match booking {
            CategorizedBooking::Warmmiete(booking) => println!("{:?}", booking),
            _ => (),
        }
    }
    Ok(())
}
//    let data = r#"
//        Warmmiete:
//            empfaenger:
//                - Horst Schlaemmer
//            verwendungszweck:
//                - 1337
//            buchungstext:
//                - DAUERAUFTRAG
//        Strom:
//            empfaenger:
//                - Hans Georg GmbH
//            verwendungszweck:
//                - Vertr. 421337
//        Internet:
//            empfaenger:
//                - Deutsche Schmelekom
//                - Kd.Nr. 13371337
//            iban:
//                - DE133713374242
//        "#;
