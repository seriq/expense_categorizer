use crate::model::*;

pub fn categorize_booking(booking: Booking, rules: &Vec<CategoryWithRule>) -> CategorizedBooking {
    for rule in rules {
        if rule.check(&booking) {
            return rule.apply(booking);
        }
    }
    CategorizedBooking {
        booking: booking,
        category: "Uncategorized".into(),
    }
}
