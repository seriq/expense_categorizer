use crate::model::*;

pub fn categorize_booking(booking: Booking, rules: &Vec<CategoryWithRule>) -> CategorizedBooking {
    for rule in rules {
        if rule.check(&booking) {
            return rule.apply(booking);
        }
    }
    CategorizedBooking::Uncategorized(booking)
}
