use crate::categorizer::categorize_booking;
use crate::parser::{parse_bookings_from_file, parse_rules_from_file, write_output_to_file};
use currency::Currency;
use itertools::Itertools;
use model::CategoryWithValue;
use num::ToPrimitive;
use std::{collections::HashMap, error::Error};

mod categorizer;
mod model;
mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules_from_file();
    let bookings = parse_bookings_from_file();
    let categorized_bookings = bookings
        .into_iter()
        .map(|booking| categorize_booking(booking, &rules))
        .sorted_by_key(|booking| booking.category.to_owned())
        .collect_vec();

    let categories_to_values: HashMap<String, Currency> = categorized_bookings
        .iter()
        .group_by(|booking| booking.category.to_owned())
        .into_iter()
        .map(|(category, group)| {
            (
                category,
                group
                    .into_iter()
                    .map(|categorized_booking| {
                        categorized_booking.booking.betrag.to_owned().unwrap()
                    })
                    .map(|betrag| {
                        Currency::from_str(&("€".to_owned() + &betrag)).expect("parse error")
                    })
                    .fold(Currency::from_str("€0").unwrap(), |a, b| a + b),
            )
        })
        .collect();
    let categories_with_values = rules
        .iter()
        .map(|rule| CategoryWithValue {
            category: rule.category.to_owned(),
            value: categories_to_values
                .get(&rule.category)
                .unwrap_or(&Currency::from_str("€0").unwrap())
                .value()
                .to_f32()
                .unwrap()
                / 100.,
        })
        .sorted_by_key(|category_with_value| category_with_value.category.to_owned())
        .collect_vec();

    write_output_to_file(categories_with_values);
    Ok(())
}
