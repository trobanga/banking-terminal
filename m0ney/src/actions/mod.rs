extern crate clap;
use clap::{ArgMatches};
use std::collections::HashMap;

use std::error::Error;
use std::fs::{File, OpenOptions};
//extern crate csv;

use chrono;
use serde::Serialize;

enum Actions {
    Show,
    Add,
    Spend,
    Borrow,
    Repay,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    datetime: &'a str,
    action: &'a str,
    amount: &'a str,
    description: &'a str,
}

pub fn perform_action(matches: &ArgMatches, file: &str) -> Result<(), Box<dyn Error>> {
    match action(&matches) {
        Some(Actions::Show) => show(&file),
        Some(Actions::Add) => add(&matches, &file),
        Some(Actions::Borrow) => borrow(&matches, &file),
        _ => Ok(()),
    }
}

fn action(matches: &ArgMatches) -> Option<Actions> {
    if matches.is_present("show") {
        return Some(Actions::Show);
    } else {
        return match matches.subcommand_name() {
            Some("add") => Some(Actions::Add),
            Some("spend") => Some(Actions::Spend),
            Some("borrow") => Some(Actions::Borrow),
            Some("repay") => Some(Actions::Repay),
            _ => None,
        };
    }
}

fn show(file: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(&file)?;
    let mut rdr = csv::Reader::from_reader(f);

    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        type Record = HashMap<String, String>;
        let record: Record = result?;
        println!("{:?}", record);
    }
    
    Ok(())
}

fn add<'a>(matches: &ArgMatches<'a>, file: &str) -> Result<(), Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("add") {
        let action = "add";
        let amount = matches.value_of("amount").unwrap();
        let category = matches.value_of("category").unwrap();
        let now = chrono::offset::Local::now();

        let record = Record {
            datetime: &now.to_string(),
            action: action,
            amount: amount,
            description: category
        };

        add_record_to_file(&record, file)?;
    }
    
    Ok(())
}

fn add_record_to_file(record: &Record, file: &str) -> Result<(), Box<dyn Error>> {
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(f);
    wtr.serialize(record)?;
    wtr.flush()?;

    Ok(())
}


fn borrow(matches: &ArgMatches, file: &str) -> Result<(), Box<dyn Error>> {

    Ok(())
}

