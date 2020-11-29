use crate::categorizer::*;
use crate::model::*;
use crate::parser::*;
use std::collections::HashMap;
use std::error::Error;

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let parsed_bookings = parse_bookings_from_file().expect("parse error");
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
    let parsed_rules = parse_rules_from_file().expect("parse error");

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
//        Telefon/Internet:
//            empfaenger:
//                - Deutsche Schmelekom
//                - Kd.Nr. 13371337
//            iban:
//                - DE133713374242
//        "#;
//    Ok(())
//}
