use crate::model::*;
use chrono::*;
use csv::{ReaderBuilder, StringRecord};
use currency::Currency;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const BOOKINGS_FILENAME: &str = "exampledata/Buchungsliste.csv";
const RULES_FILENAME: &str = "exampledata/Rules.yaml";

pub fn parse_bookings_from_file() -> csv::Result<Vec<Booking>> {
    let file = File::open(BOOKINGS_FILENAME).expect("Error while opening file.");
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut parsed_bookings = Vec::new();
    for result in rdr.records() {
        let entry = parse_entry(result?);
        if entry.buchungsdatum >= NaiveDate::from_ymd(2020, 08, 01) {
            parsed_bookings.push(entry);
        }
    }
    Ok(parsed_bookings)
}

pub fn parse_rules_from_file() -> Result<(), serde_yaml::Error> {
    let file = File::open(RULES_FILENAME).expect("Error while opening file.");
    let buf_reader = BufReader::new(file);
    let iterator =
        serde_yaml::from_reader::<_, HashMap<String, BookingRule>>(buf_reader).into_iter();
    for thing in iterator {
        dbg!(thing);
    }
    Ok(())
}

fn parse_entry(record: StringRecord) -> Booking {
    Booking {
        buchungsdatum: parse_naive_date(&record[0]),
        empfaenger: String::from(&record[1]),
        verwendungszweck: String::from(&record[2]),
        buchungstext: String::from(&record[3]),
        betrag: parse_currency(&record[4]),
        iban: String::from(&record[5]),
        bic: String::from(&record[6]),
        kategorie: String::from(&record[7]),
        notiz: String::from(&record[9]),
        schlagworte: String::from(&record[9]),
    }
}

fn parse_naive_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(&date, "%d.%m.%Y").expect("parse error")
}

fn parse_currency(currency: &str) -> Currency {
    Currency::from_str(&["€", currency].join("")).expect("parse error")
}
