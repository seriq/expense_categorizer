extern crate csv;

use csv::ReaderBuilder;
use std::error::Error;
use std::fmt::Display;

fn main() {}

#[derive(Debug)]
enum AccountEntry {
    Account(DkbAccountEntry),
    Visa(DkbVisaEntry),
    Amex(AmexEntry),
}

impl Display for AccountEntry {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Beschreibung: {}, Betrag: {}",
            self.beschreibung, self.betrag
        );
    }
}

#[derive(Default, Debug)]
struct DkbAccountEntry {
    belegdatum: String,   //TODO: date
    wertstellung: String, //TODO: date
    buchungstext: String,
    auftraggeber_beguenstigter: String,
    beschreibung: String, //verwendungszweck
    kontonummer: String,  //TODO: IBAN
    blz: String,          //TODO: BIC
    betrag: String,       //TODO: EUR
    glaeubiger_id: String,
    mandatsreferenz: String,
    kundenreferenz: String,
}

#[derive(Default, Debug)]
struct DkbVisaEntry {
    umsatz_abgerechnet_und_nicht_im_saldo_enthalten: bool,
    wertstellung: String, //TODO: date
    belegdatum: String,   //TODO: date
    beschreibung: String,
    betrag: String,                 //TODO: EUR
    urspruenglicher_betrag: String, //TODO: EUR
}

#[derive(Default, Debug)]
struct AmexEntry {
    belegdatum: String, //TODO: date
    referenz: String,
    betrag: String, //TODO: EUR
    beschreibung: String,
    wertstellung: String, //TODO: date
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_string() {
        let buchung = AccountEntry::Account(DkbAccountEntry {
            auftraggeber_beguenstigter: String::from("1337"),
            verwendungszweck: String::from("This is an interesting booking."),
            ..Default::default()
        });
        assert_eq!(
            format!("{:#?}", buchung),
            String::from(
                r#"Account(DkbAccountEntry { buchungstag: "",
                                           wertstellung: "",
                                           buchungstext: "",
                                           auftraggeber_beguenstigter: "1337",
                                           verwendungszweck: "This is an interesting booking.",
                                           kontonummer: "",
                                           blz: "",
                                           betrag: "",
                                           glaeubiger_id: "",
                                           mandatsreferenz: "",
                                           kundenreferenz: "" })"#
            )
        )
    }
}
