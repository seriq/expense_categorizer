use crate::model::*;
use chrono::*;
use csv::{ReaderBuilder, StringRecord};
use currency::Currency;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;

pub fn parse_file() -> csv::Result<()> {
    let file = File::open("exampledata/account_noheader.csv")?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(transcoded);
    for result in rdr.records() {
        let entry = parse_dkb_account_entry(result?);
        if entry.wertstellung >= NaiveDate::from_ymd(2020, 08, 31) {
            print!("{:#?}", entry.betrag / 100);
        }
    }
    Ok(())
}

fn parse_dkb_account_entry(record: StringRecord) -> Booking {
    Booking {
        beschreibung: String::from(&record[2]),
        betrag: Currency::from_str(&["€", &record[7]].join("")).expect("parse error"),
        belegdatum: NaiveDate::parse_from_str(&record[0], "%d.%m.%Y").expect("parse error"),
        wertstellung: NaiveDate::parse_from_str(&record[1], "%d.%m.%Y").expect("parse error"),
        additional_details: Default::default(),
    }
}
