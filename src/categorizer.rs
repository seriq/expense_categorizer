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
    categorized_bookings: Vec<CategorizedBooking>,
) -> HashMap<String, Currency> {
    categorized_bookings
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
        .collect()
}
