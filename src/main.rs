extern crate csv;

//use csv::ReaderBuilder;
//use std::error::Error;
use std::fmt;

fn main() {}

#[derive(Default, Debug)]
struct Booking {
    beschreibung: String,
    betrag: String,       //TODO: EUR
    belegdatum: String,   //TODO: date
    wertstellung: String, //TODO: date
    additional_details: AdditionalDetails,
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

#[derive(Debug)]
enum AdditionalDetails {
    DkbAccount(DkbAccountDetails),
    DkbVisa(DkbVisaDetails),
    Amex(AmexDetails),
}

impl Default for AdditionalDetails {
    fn default() -> Self {
        AdditionalDetails::DkbAccount(DkbAccountDetails::default())
    }
}

#[derive(Default, Debug)]
struct DkbAccountDetails {
    buchungstext: String,
    auftraggeber_beguenstigter: String,
    kontonummer: String, //TODO: IBAN
    blz: String,         //TODO: BIC
    glaeubiger_id: String,
    mandatsreferenz: String,
    kundenreferenz: String,
}

#[derive(Default, Debug)]
struct DkbVisaDetails {
    umsatz_abgerechnet_und_nicht_im_saldo_enthalten: bool,
    urspruenglicher_betrag: String, //TODO: EUR
}

#[derive(Default, Debug)]
struct AmexDetails {
    referenz: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let buchung_dkb = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: String::from("13,37 EUR"),
            belegdatum: String::from("20.04.2020"),
            wertstellung: String::from("21.04.2020"),
            ..Default::default()
        };
        let buchung_visa = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: String::from("13,37 EUR"),
            belegdatum: String::from("20.04.2020"),
            wertstellung: String::from("21.04.2020"),
            additional_details: AdditionalDetails::DkbVisa(DkbVisaDetails::default()),
        };
        let buchung_amex = Booking {
            beschreibung: String::from("This is an interesting booking."),
            betrag: String::from("13,37 EUR"),
            belegdatum: String::from("20.04.2020"),
            wertstellung: String::from("21.04.2020"),
            additional_details: AdditionalDetails::Amex(AmexDetails::default()),
        };
        print!("{:#?}", buchung_dkb);
        print!("{:#?}", buchung_visa);
        print!("{:#?}", buchung_amex);
    }
}
