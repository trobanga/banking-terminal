use std::error::Error;
use std::fs::{File, OpenOptions};

use chrono;
use serde::{Deserialize, Serialize};

pub mod accounts;
pub use accounts::Accounts;

pub mod matches;
use matches::Commands;
pub use matches::{parse_matches, Matches};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_new() {
        let record = Record::new(
            &Commands::Spend,
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
    command: String,
    amount: f32,
    description: String,
    balance: f32,
}

impl Record {
    fn new(
        command: &Commands,
        amount: f32,
        description: String,
        old_balance: f32,
    ) -> Result<Self, Box<dyn Error>> {
        let datetime = chrono::offset::Local::now().to_string();
        let command = command.name_string();
        let balance = ((old_balance + amount) * 100.).round() / 100.;
        Ok(Record {
            datetime,
            command,
            amount,
            description,
            balance,
        })
    }
}

pub fn apply(matches: &Matches, accounts: &Accounts) -> Result<(), Box<dyn Error>> {
    match &matches.command {
        Commands::Show => show(&matches, &accounts),
        Commands::Accounts => {
            if matches.subcommands.contains_key("list") {
                return accounts.list();
            } else if let Some(x) = matches.subcommands.get("new") {
                return accounts.new_account(&matches.config_file, &x);
            } else if let Some(x) = matches.subcommands.get("delete") {
                println!("{:?}", x);
            }
            return Err("Cannot perform command.".into());
        }
        command => create_record_and_save_to_file(&matches, &accounts, &command),
    }
}

fn show(matches: &Matches, accounts: &Accounts) -> Result<(), Box<dyn Error>> {
    let account = account(&matches)?;
    if account == "__ALL__" {
        for account in accounts.accounts.keys() {
            show_account(&accounts, &account)?;
        }
    } else {
        show_account(&accounts, &account)?;
    }
    Ok(())
}

fn show_account(accounts: &Accounts, account: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(&accounts.accounts[account]).expect("Account does not exist.");
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn create_record_and_save_to_file(
    matches: &Matches,
    accounts: &Accounts,
    command: &Commands,
) -> Result<(), Box<dyn Error>> {
    let account = &accounts.accounts[&account(&matches)?];
    let description = description(&matches)?;
    let amount: f32 = amount(&matches, &command)?;
    let balance = balance(&account)?;
    let record = Record::new(&command, amount, description, balance)?;
    add_record_to_file(&record, &account)
}

fn get_match_value(name: &str, matches: &Matches) -> Result<String, Box<dyn Error>> {
    matches.get(name)
}

fn account(matches: &Matches) -> Result<String, Box<dyn Error>> {
    get_match_value("account", &matches)
}

fn description(matches: &Matches) -> Result<String, Box<dyn Error>> {
    get_match_value("description", &matches)
}

fn amount(matches: &Matches, command: &Commands) -> Result<f32, Box<dyn Error>> {
    let amount = get_match_value("amount", &matches)
        .unwrap()
        .parse::<f32>()?;
    Ok(&command.sign()? * amount)
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
