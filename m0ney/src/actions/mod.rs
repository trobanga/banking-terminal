use clap::{ArgMatches};
use std::collections::HashMap;

use std::error::Error;
use std::fs::{File, OpenOptions};

use chrono;
use serde::{Serialize, Deserialize};
//use serde::de::DeserializeOwned;

enum Actions {
    Show,
    Add,
    Spend,
    Borrow,
    Repay,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record<'a> {
    datetime: String,
    action: &'a str,
    amount: f64,
    description: &'a str,
    balance: f64
}


impl<'a> Record<'a> {
    fn new(action: &'a str,
           amount: f64,
           description: &'a str,
           file: &'a str) -> Self {
        let datetime = chrono::offset::Local::now().to_string();
        let mut rdr = csv::Reader::from_path(file).unwrap();
        if let Some(x) = rdr.records().last().unwrap().unwrap().get(4) {
            let balance = amount + x.parse::<f64>().unwrap();
            return Record {datetime, action, amount, description, balance};        
        }
        panic!("Could not determine new balance.");
        Record {datetime, action, amount, description, balance: 0.0}
    }
}

// impl Record<'_> {
//     fn new<'a> (datetime: &'a str,
//                 action: &'a str,
//                 amount: f64,
//                 description: &'a str,
//                 file: &'a str) -> Self {
//         let mut rdr = csv::Reader::from_path(file).unwrap();
//         let iter = rdr.deserialize();
//         let balance = iter.last().unwrap().unwrap();
//         let d = &String::from(datetime)[..];
//         let a = &String::from(action);
//         let dd = &String::from(description);
//         Record {datetime: d, action: a, amount: amount, description: dd, balance: balance}
//     }
// }

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
        let amount: f64 = matches.value_of("amount").unwrap().parse::<f64>()?;
        let category = matches.value_of("category").unwrap();
        
        let record = Record::new(
            action,
            amount,
            category,
            file
        );
        
        add_record_to_file(&record, file)?;
    }
    
    Ok(())
}

fn add_record_to_file(record: &Record, file: &str) -> Result<(), Box<dyn Error>> {
    let f = OpenOptions::new()
        .read(true)
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

