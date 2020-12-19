use std::collections::HashMap;

use currency::Currency;
use itertools::Itertools;

use crate::model::*;

pub fn categorize_bookings(
    bookings: Vec<Booking>,
    rules: &Vec<CategoryWithRule>,
) -> Vec<CategorizedBooking> {
    bookings
        .into_iter()
        .map(|booking| categorize_booking(booking, &rules))
        .sorted_by_key(|booking| booking.category.to_owned())
        .collect_vec()
}

fn categorize_booking(booking: Booking, rules: &Vec<CategoryWithRule>) -> CategorizedBooking {
    let applicable_rules: Vec<&CategoryWithRule> = rules
        .into_iter()
        .filter(|rule| rule.check(&booking))
        .collect();

    match applicable_rules.len() {
        1 => applicable_rules.get(0).unwrap().apply(booking),
        0 => CategorizedBooking {
            category: "Andere Ausgaben".into(),
            booking,
        },
        _ => CategorizedBooking {
            category: "Multiple Matches".into(),
            booking,
        },
    }
}
pub fn collect_categories_to_values(
    categories_to_values: &HashMap<String, Vec<CategorizedBooking>>,
) -> HashMap<String, Currency> {
    categories_to_values
        .iter()
        .map(|(category, bookings)| {
            (
                category.to_owned(),
                bookings
                    .iter()
                    .map(|categorized_booking| {
                        match categorized_booking.booking.betrag.to_owned() {
                            Some(betrag) => betrag,
                            None => panic!(
                                "The booking {:?} has no betrag specified.",
                                categorized_booking.booking
                            ),
                        }
                    })
                    .map(|betrag| {
                        Currency::from_str(&("€".to_owned() + &betrag)).expect("parse error")
                    })
                    .fold(Currency::from_str("€0").unwrap(), |a, b| a + b),
            )
        })
        .collect()
}

pub fn group_bookings_by_categories(
    categorized_bookings: Vec<CategorizedBooking>,
) -> HashMap<String, Vec<CategorizedBooking>> {
    categorized_bookings
        .into_iter()
        .map(|categorized_booking| (categorized_booking.category.to_owned(), categorized_booking))
        .into_group_map()
}
