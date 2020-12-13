use crate::categorizer::categorize_bookings;
use crate::parser::{parse_bookings_from_file, parse_rules_from_file, write_output_to_file};
use categorizer::collect_categories_to_values;
use currency::Currency;
use std::{collections::HashMap, error::Error};

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules_from_file();
    let bookings = parse_bookings_from_file();
    let categorized_bookings = categorize_bookings(bookings, &rules);

    let categories_to_values: HashMap<String, Currency> =
        collect_categories_to_values(categorized_bookings);

    write_output_to_file(rules, categories_to_values);
    Ok(())
}
