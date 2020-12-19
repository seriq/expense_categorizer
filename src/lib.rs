use crate::categorizer::categorize_bookings;
use crate::parser::{parse_bookings_from_file, parse_rules_from_file, write_output_to_file};
use categorizer::{collect_categories_to_values, group_bookings_by_categories};
use currency::Currency;
use parser::write_left_over_bookings_to_file;
use std::{collections::HashMap, error::Error};

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules_from_file();
    let bookings = parse_bookings_from_file();
    let categorized_bookings = categorize_bookings(bookings, &rules);

    let categories_to_bookings = group_bookings_by_categories(categorized_bookings);
    let categories_to_values: HashMap<String, Currency> =
        collect_categories_to_values(&categories_to_bookings);

    write_output_to_file(rules, categories_to_values);
    write_left_over_bookings_to_file(&categories_to_bookings["Andere Ausgaben"]);
    Ok(())
}
