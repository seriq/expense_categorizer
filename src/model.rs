use serde::Deserialize;

//TODO: Booking und bookingrule zusammenfassen
#[derive(Debug, PartialEq, Eq, Default, Deserialize)]
pub struct Booking {
    #[serde(rename = "Buchungsdatum")]
    pub buchungsdatum: Option<String>,
    #[serde(rename = "Empfaenger")]
    pub empfaenger: Option<String>,
    #[serde(rename = "Verwendungszweck")]
    pub verwendungszweck: Option<String>,
    #[serde(rename = "Buchungstext")]
    pub buchungstext: Option<String>,
    #[serde(rename = "Betrag")]
    pub betrag: Option<String>,
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
    #[serde(rename = "BIC")]
    pub bic: Option<String>,
    #[serde(rename = "Kategorie")]
    pub kategorie: Option<String>,
    #[serde(rename = "Notiz")]
    pub notiz: Option<String>,
    #[serde(rename = "Schlagworte")]
    pub schlagworte: Option<String>,
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
    Strom(Booking),
    Internet(Booking),
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
