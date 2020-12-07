use crate::model::*;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

const BOOKINGS_FILENAME: &str = "exampledata/Buchungsliste.csv";
const RULES_FILENAME: &str = "exampledata/Rules.yaml";

pub fn parse_bookings_from_file() -> csv::Result<Vec<Booking>> {
    let file = File::open(BOOKINGS_FILENAME)?;
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
    rdr.deserialize().collect()
}

pub fn parse_rules_from_file() -> Result<Vec<CategoryWithRule>, Box<dyn Error>> {
    let parsed_rules = parse_booking_rules_from_file();
    let mut result = Vec::new();

    for (category, rule) in parsed_rules? {
        result.push(CategoryWithRule {
            category: category,
            booking_rule: rule,
        })
    }
    Ok(result)
}

fn parse_booking_rules_from_file() -> Result<HashMap<String, BookingRule>, serde_yaml::Error> {
    let file = File::open(RULES_FILENAME).expect("Error while opening file.");
    let buf_reader = BufReader::new(file);
    serde_yaml::from_reader(buf_reader)
}
