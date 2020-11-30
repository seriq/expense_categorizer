use crate::categorizer::*;
use crate::model::*;
use crate::parser::*;
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let parsed_rules = parse_rules_from_file().expect("parse error");
    let rules = create_rules(parsed_rules);
    let parsed_bookings = parse_bookings_from_file().expect("parse error");
    let mut categorized_bookings: Vec<CategorizedBooking> = Vec::new();
    for booking in parsed_bookings {
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
//pub fn run() -> Result<(), Box<dyn Error>> {
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
//    Ok(())
//}
