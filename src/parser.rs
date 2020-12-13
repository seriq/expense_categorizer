use crate::model::*;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const BOOKINGS_FILENAME: &str = "data/Buchungsliste.csv";
const RULES_FILENAME: &str = "data/Rules.yaml";

pub fn parse_bookings_from_file() -> csv::Result<Vec<Booking>> {
    let file = File::open(BOOKINGS_FILENAME)?;
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
    rdr.deserialize().collect()
}

pub fn parse_rules_from_file() -> Vec<CategoryWithRule> {
    let parsed_rules = parse_booking_rules_from_file();
    parsed_rules
        .into_iter()
        .map(|(category, booking_rule)| CategoryWithRule {
            category,
            booking_rule,
        })
        .collect()
}

fn parse_booking_rules_from_file() -> HashMap<String, BookingRule> {
    let file = File::open(RULES_FILENAME).expect("Error while opening file.");
    serde_yaml::from_reader(BufReader::new(file)).expect("Error while parsing rules from file.")
}
