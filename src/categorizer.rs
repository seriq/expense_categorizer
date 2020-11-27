use crate::model::Booking;

pub fn categorize_booking(booking: Booking) -> CategorizedBooking {
    for rule in create_rules() {
        if rule.check(&booking) {
            return rule.apply(booking);
        }
    }
    CategorizedBooking::Uncategorized(booking)
}

pub fn detect_warmmiete(booking: &Booking) -> bool {
    booking.beschreibung.contains("Miete")
}

#[derive(Debug, PartialEq, Eq)]
pub enum CategorizedBooking {
    Warmmiete(Booking),
    Uncategorized(Booking),
}

fn create_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    rules.push(Rule {
        check: detect_warmmiete,
        apply: CategorizedBooking::Warmmiete,
    });
    rules
}

struct Rule {
    check: fn(&Booking) -> bool,
    apply: fn(Booking) -> CategorizedBooking,
}

impl Rule {
    fn check(&self, booking: &Booking) -> bool {
        (self.check)(booking)
    }

    fn apply(&self, booking: Booking) -> CategorizedBooking {
        (self.apply)(booking)
    }
}
