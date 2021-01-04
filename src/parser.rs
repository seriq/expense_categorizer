use crate::model::*;
use csv::{ReaderBuilder, WriterBuilder};
use currency::Currency;
use itertools::Itertools;
use num::ToPrimitive;
use std::{collections::HashMap, fs::File, io::BufReader};

const BOOKINGS_FILENAME: &str = "data/Buchungsliste.csv";
const RULES_FILENAME: &str = "data/Rules.yaml";
const OUTPUT_FILENAME: &str = "data/Output.csv";
const LEFT_OVERS_FILENAME: &str = "data/LeftOvers.csv";

pub fn parse_bookings_from_file() -> Vec<Booking> {
    let file = File::open(BOOKINGS_FILENAME)
        .expect(&format!("Error while opening file {}", BOOKINGS_FILENAME));
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);
    reader
        .deserialize()
        .collect::<csv::Result<Vec<Booking>>>()
        .expect("Eror while parsing bookings from file.")
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

pub fn write_output_to_file(
    rules: &Vec<CategoryWithRule>,
    categories_to_values: &HashMap<String, Currency>,
) {
    let file = File::create(OUTPUT_FILENAME).expect("Error creating Output file.");
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(file);

    let categories_with_values = prepare_data_for_output(rules, categories_to_values);
    for record in &categories_with_values {
        writer
            .serialize(record)
            .expect(&format!("Error writing record {:?}", record));
    }
    writer.flush().expect("Error flushing writer.");
}

pub fn handle_skipped_bookings(categories_to_bookings: &HashMap<String, Vec<CategorizedBooking>>) {
    let trash_bookings = categories_to_bookings
        .get("Trash")
        .expect("No Trash bookings. Possibly something went wrong.");
    for categorized_booking in trash_bookings {
        println!("Dropping booking {:?}", categorized_booking.booking);
    }
    write_left_over_bookings_to_file(&categories_to_bookings["Andere Ausgaben"]);
}

fn write_left_over_bookings_to_file(categorized_bookings: &Vec<CategorizedBooking>) {
    let file = File::create(LEFT_OVERS_FILENAME).expect("Error creating LeftOvers file.");
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(file);

    for record in categorized_bookings {
        writer
            .serialize(&record.booking)
            .expect(&format!("Error writing record {:?}", record));
    }
    writer.flush().expect("Error flushing writer.");
}

fn parse_booking_rules_from_file() -> HashMap<String, BookingRule> {
    let file =
        File::open(RULES_FILENAME).expect(&format!("Error while opening file {}", RULES_FILENAME));
    serde_yaml::from_reader(BufReader::new(file)).expect("Error while parsing rules from file.")
}

fn prepare_data_for_output(
    rules: &Vec<CategoryWithRule>,
    categories_to_values: &HashMap<String, Currency>,
) -> Vec<CategoryWithValue> {
    rules
        .iter()
        .map(|rule| CategoryWithValue {
            category: rule.category.to_owned(),
            value: categories_to_values
                .get(&rule.category)
                .and_then(|currency| currency.value().to_f32().map(|value| -value / 100.)),
        })
        .sorted_by_key(|category_with_value| category_with_value.category.to_owned())
        .collect()
}
