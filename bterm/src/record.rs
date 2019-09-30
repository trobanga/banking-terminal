use clap::ArgMatches;

use std::error::Error;
use std::fs::{File, OpenOptions};

use chrono;
use serde::{Deserialize, Serialize};

mod actions;
use actions::Actions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_new() {
        let record = Record::new(
            &Actions::Spend,
            -138.95,
            String::from("more m0ney"),
            2405.83,
        )
        .unwrap();
        assert_eq!(record.amount, -138.95);
        assert_eq!(record.balance, 2266.88);
        assert_eq!(record.description, String::from("more m0ney"));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    datetime: String,
    action: String,
    amount: f32,
    description: String,
    balance: f32,
}

impl Record {
    fn new(
        action: &Actions,
        amount: f32,
        description: String,
        old_balance: f32,
    ) -> Result<Self, Box<dyn Error>> {
        let datetime = chrono::offset::Local::now().to_string();
        let action = action.name_string();
        let balance = ((old_balance + amount) * 100.).round() / 100.;
        Ok(Record {
            datetime,
            action,
            amount,
            description,
            balance,
        })
    }
}

pub fn apply(matches: &ArgMatches, file: &str) -> Result<(), Box<dyn Error>> {
    if let Some(Actions::Show) = Actions::from_matches(&matches) {
        show(&file)
    } else {
        let record = match Actions::from_matches(&matches) {
            Some(action) => {
                let description = description(&matches, &action)?;
                let amount: f32 = amount(&matches, &action)?;
                let balance = balance(&file)?;

                Record::new(&action, amount, description, balance)
            }
            _ => Err("Not implemented".into()),
        }?;

        add_record_to_file(&record, &file)
    }
}

fn show(file: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(&file)?;
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn description(matches: &ArgMatches, action: &Actions) -> Result<String, Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches(&action.name_string()) {
        let description = matches.value_of(action.description_string()).unwrap();
        return Ok(String::from(description));
    }
    Err("Could not determine description.".into())
}

fn amount(matches: &ArgMatches, action: &Actions) -> Result<f32, Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches(&action.name_string()) {
        let amount = matches.value_of("amount").unwrap().parse::<f32>()?;
        return Ok(&action.sign()? * amount);
    }
    Err("Could not determine amount.".into())
}

fn balance(file: &str) -> Result<f32, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file)?;
    if let Some(x) = rdr.records().last().unwrap()?.get(4) {
        let balance = x.parse::<f32>()?;
        return Ok(balance);
    }
    Err("Could not read balance.".into())
}

fn add_record_to_file(record: &Record, file: &str) -> Result<(), Box<dyn Error>> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(file)?;

    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(f);
    wtr.serialize(record)?;
    wtr.flush()?;
    Ok(())
}