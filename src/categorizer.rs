use crate::model::*;
use std::collections::HashMap;

pub fn categorize_booking(booking: Booking, rules: &Vec<Rule>) -> CategorizedBooking {
    for rule in rules {
        if rule.check(&booking) {
            return rule.apply(booking);
        }
    }
    CategorizedBooking::Uncategorized(booking)
}

pub fn create_rules(booking_rules: HashMap<String, BookingRule>) -> Vec<Rule> {
    let mut rules = Vec::new();
    for (name, rule) in booking_rules {
        rules.push(Rule {
            check: build_rule(rule),
            apply: find_constructor_for_name(name),
        });
    }
    rules
}

fn build_rule(booking_rule: BookingRule) -> fn(&Booking) -> bool {
    |booking: &Booking| true
}

fn find_constructor_for_name(name: String) -> fn(Booking) -> CategorizedBooking {
    match name.as_str() {
        "Warmmiete" => CategorizedBooking::Warmmiete,
        "Strom" => CategorizedBooking::Strom,
        "Internet" => CategorizedBooking::Internet,
        _ => CategorizedBooking::Uncategorized,
    }
}
