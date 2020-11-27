use chrono::NaiveDate;
use currency::Currency;

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
