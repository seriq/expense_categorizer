use crate::model::*;
use std::collections::HashMap;

use currency::Currency;
use itertools::Itertools;

pub fn categorize_bookings(
    bookings: Vec<Booking>,
    rules: &Vec<CategoryWithRule>,
) -> Vec<CategorizedBooking> {
    bookings
        .into_iter()
        .map(|booking| categorize_booking(booking, &rules))
        .sorted_by_key(|booking| booking.category.to_owned())
        .collect()
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
        _ => {
            println!(
                "Warning! The booking {:?} matches multiple categories!",
                booking
            );
            CategorizedBooking {
                category: "Multiple Matches".into(),
                booking,
            }
        }
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

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_categorize_bookings() {
        //given
        let bookings = generate_test_bookings();
        let rules = generate_test_rules();

        //when
        let categorized_bookings = categorize_bookings(bookings, &rules);

        //then
        let categories = categorized_bookings
            .into_iter()
            .map(|booking| booking.category)
            .collect_vec();

        assert!(categories.contains(&"Multiple Matches".to_owned()));
        assert!(categories.contains(&"Andere Ausgaben".to_owned()));
        assert!(categories.contains(&"Warmmiete".to_owned()));
    }

    fn generate_test_bookings() -> Vec<Booking> {
        let booking1 = Booking {
            buchungsdatum: Some("01.01.2011".to_owned()),
            empfaenger: Some("Horst Schlämmer".to_owned()),
            verwendungszweck: Some("Miete".to_owned()),
            ..Default::default()
        };
        let i_wont_match_any_rule = Booking {
            buchungsdatum: Some("01.01.2011".to_owned()),
            empfaenger: Some("Horst Georg GmbH".to_owned()),
            ..Default::default()
        };
        let i_will_match_multiple_rules = Booking {
            empfaenger: Some("Horst Schlämmer".to_owned()),
            betrag: Some("1000".to_owned()),
            ..Default::default()
        };
        vec![booking1, i_wont_match_any_rule, i_will_match_multiple_rules]
    }

    fn generate_test_rules() -> Vec<CategoryWithRule> {
        let booking_rule1 = BookingRule {
            buchungsdatum: Some(vec!["01.01.2011".to_owned()]),
            empfaenger: Some(vec![
                "Horst Schlämmer".to_owned(),
                "Hans Georg Maaßen".to_owned(),
            ]),
            ..Default::default()
        };
        let booking_rule2 = BookingRule {
            betrag: Some(vec!["1000".to_owned()]),
            ..Default::default()
        };
        vec![
            CategoryWithRule {
                category: "Warmmiete".to_owned(),
                booking_rule: booking_rule1,
            },
            CategoryWithRule {
                category: "Ein k lol".to_owned(),
                booking_rule: booking_rule2,
            },
        ]
    }
}
