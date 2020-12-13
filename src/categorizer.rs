use crate::model::*;

pub fn categorize_booking(booking: Booking, rules: &Vec<CategoryWithRule>) -> CategorizedBooking {
    let applicable_rules: Vec<&CategoryWithRule> = rules
        .into_iter()
        .filter(|rule| rule.check(&booking))
        .collect();

    match applicable_rules.len() {
        1 => applicable_rules.get(0).unwrap().apply(booking),
        0 => CategorizedBooking {
            category: "Uncategorized".into(),
            booking,
        },
        _ => CategorizedBooking {
            category: "Multiple Matches".into(),
            booking,
        },
    }
}
