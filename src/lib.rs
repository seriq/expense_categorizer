use categorizer::{
    categorize_bookings, collect_categories_to_values, group_bookings_by_categories,
};
use currency::Currency;
use parser::{
    handle_skipped_bookings, parse_bookings_from_file, parse_rules_from_file, write_output_to_file,
};
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
    handle_skipped_bookings(&categories_to_bookings);
    Ok(())
}
