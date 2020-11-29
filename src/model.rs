use chrono::NaiveDate;
use currency::Currency;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub struct Booking {
    pub buchungsdatum: NaiveDate,
    pub empfaenger: String,
    pub verwendungszweck: String,
    pub buchungstext: String,
    pub betrag: Currency,
    pub iban: String,
    pub bic: String,
    pub kategorie: String,
    pub notiz: String,
    pub schlagworte: String,
}

#[derive(Debug, Deserialize)]
pub struct BookingRule {
    pub buchungsdatum: Option<Vec<String>>,
    pub empfaenger: Option<Vec<String>>,
    pub verwendungszweck: Option<Vec<String>>,
    pub buchungstext: Option<Vec<String>>,
    pub betrag: Option<Vec<String>>,
    pub iban: Option<Vec<String>>,
    pub bic: Option<Vec<String>>,
    pub kategorie: Option<Vec<String>>,
    pub notiz: Option<Vec<String>>,
    pub schlagworte: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CategorizedBooking {
    Warmmiete(Booking),
    Uncategorized(Booking),
}

pub struct Rule {
    pub check: fn(&Booking) -> bool,
    pub apply: fn(Booking) -> CategorizedBooking,
}

impl Rule {
    pub fn check(&self, booking: &Booking) -> bool {
        (self.check)(booking)
    }

    pub fn apply(&self, booking: Booking) -> CategorizedBooking {
        (self.apply)(booking)
    }
}
