use crate::Booking;
use csv::{ReaderBuilder, StringRecord};
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
        print!("{:#?}", entry.betrag);
    }
    Ok(())
}

fn parse_dkb_account_entry(record: StringRecord) -> Booking {
    Booking {
        beschreibung: String::from(&record[2]),
        betrag: String::from(&record[7]),
        belegdatum: String::from(&record[0]),
        wertstellung: String::from(&record[1]),
        ..Default::default()
    }
}
