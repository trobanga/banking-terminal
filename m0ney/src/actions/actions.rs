extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
extern crate csv;

use chrono;

enum Actions {
    Show,
    Add,
    Spend,
    Borrow,
    Repay,
}

pub fn perform_action(matches: &ArgMatches, file: &str) -> Result<(), Box<dyn Error>> {
    match action(&matches) {
        Some(Actions::Show) => show(&file),
        Some(Actions::Add) => add(&matches, &file),
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

fn add(matches: &ArgMatches, file: &str) -> Result<(), Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("add") {
        let amount = matches.value_of("amount").unwrap();
        let category = matches.value_of("category").unwrap();
        let mut now = chrono::offset::Local::now();
        println!("{:?}", now.format("%F %T").to_string());
        
    }
    
    Ok(())
}
