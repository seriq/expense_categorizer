use chrono::NaiveDate;
use currency::Currency;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Booking {
    pub beschreibung: String,
    pub betrag: Currency,
    pub belegdatum: NaiveDate,
    pub wertstellung: NaiveDate,
    pub additional_details: AdditionalDetails,
}

impl fmt::Display for Booking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Beschreibung: {}, Betrag: {}, Belegdatum: {}, Wertstellung: {}",
            self.beschreibung, self.betrag, self.belegdatum, self.wertstellung
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AdditionalDetails {
    DkbAccount(DkbAccountDetails),
    DkbVisa(DkbVisaDetails),
    Amex(AmexDetails),
}

impl Default for AdditionalDetails {
    fn default() -> Self {
        AdditionalDetails::DkbAccount(DkbAccountDetails::default())
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct DkbAccountDetails {
    buchungstext: String,
    auftraggeber_beguenstigter: String,
    kontonummer: String,
    blz: String,
    glaeubiger_id: String,
    mandatsreferenz: String,
    kundenreferenz: String,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct DkbVisaDetails {
    umsatz_abgerechnet_und_nicht_im_saldo_enthalten: bool,
    urspruenglicher_betrag: String, //TODO: EUR
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct AmexDetails {
    referenz: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    #[test]
    fn test_eq() {
        let buchung_dkb = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: parse_currency("13,37"),
            belegdatum: parse_naive_date("20.04.2020"),
            wertstellung: parse_naive_date("21.04.2020"),
            additional_details: Default::default(),
        };
        let buchung_visa = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: parse_currency("13,37"),
            belegdatum: parse_naive_date("20.04.2020"),
            wertstellung: parse_naive_date("21.04.2020"),
            additional_details: AdditionalDetails::DkbVisa(DkbVisaDetails::default()),
        };
        let buchung_amex = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: parse_currency("13,37"),
            belegdatum: parse_naive_date("20.04.2020"),
            wertstellung: parse_naive_date("21.04.2020"),
            additional_details: AdditionalDetails::Amex(AmexDetails::default()),
        };
        assert_ne!(buchung_dkb, buchung_visa);
        assert_ne!(buchung_visa, buchung_amex);
        assert_ne!(buchung_amex, buchung_dkb);
        assert_eq!(buchung_amex, buchung_amex);

        print!("{:#?}", buchung_dkb);
    }
}
