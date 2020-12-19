use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
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

impl BookingRule {
    fn check(&self, booking: &Booking) -> bool {
        BookingRule::any_present(&self.buchungstext, &booking.buchungstext)
            && BookingRule::any_present(&self.empfaenger, &booking.empfaenger)
            && BookingRule::any_present(&self.verwendungszweck, &booking.verwendungszweck)
    }

    fn any_present(needles: &Option<Vec<String>>, haystack: &Option<String>) -> bool {
        match needles {
            Some(needles) => match haystack {
                Some(haystack) => needles.iter().any(|a| haystack.contains(a)),
                None => false,
            },
            None => true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CategoryWithRule {
    pub category: String,
    pub booking_rule: BookingRule,
}

#[derive(Debug, PartialEq)]
pub struct CategorizedBooking {
    pub category: String,
    pub booking: Booking,
}

impl CategoryWithRule {
    pub fn apply(&self, booking: Booking) -> CategorizedBooking {
        CategorizedBooking {
            category: self.category.clone(),
            booking,
        }
    }
    pub fn check(&self, booking: &Booking) -> bool {
        self.booking_rule.check(booking)
    }
}
#[derive(Debug, Serialize)]
pub struct CategoryWithValue {
    pub category: String,
    pub value: Option<f32>,
}
