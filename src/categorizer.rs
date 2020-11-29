use crate::model::*;

pub fn categorize_booking(booking: Booking) -> CategorizedBooking {
    for rule in create_rules() {
        if rule.check(&booking) {
            return rule.apply(booking);
        }
    }
    CategorizedBooking::Uncategorized(booking)
}

fn detect_warmmiete(booking: &Booking) -> bool {
    booking.verwendungszweck.contains("Miete") && booking.buchungstext == "DAUERAUFTRAG"
}

fn create_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    rules.push(Rule {
        check: detect_warmmiete,
        apply: CategorizedBooking::Warmmiete,
    });
    rules
}
