use clap::ArgMatches;
use std::error::Error;

mod accounts;
use accounts::Accounts;

mod matches;
use matches::{parse_matches, Commands, Matches};

pub struct BTerm {
    matches: Matches,
    accounts: Accounts,
}

impl BTerm {
    pub fn new(matches: &ArgMatches) -> Self {
        let matches = parse_matches(&matches).unwrap();

        let accounts = Accounts::new(&matches.config_file);
        Self { matches, accounts }
    }

    pub fn apply(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.matches.command {
            Commands::Show => Ok(()), //show(&self.matches, &self.accounts),
            Commands::Accounts => {
                if self.matches.subcommands.contains_key("list") {
                    return self.accounts.list();
                } else if let Some(x) = self.matches.subcommands.get("new") {
                    return self.accounts.add_account(&self.matches.config_file, &x);
                } else if let Some(x) = self.matches.subcommands.get("delete") {
                    println!("{:?}", x);
                }
                return Err("Cannot perform command.".into());
            }
            _ => Ok(()), // command => create_record_and_save_to_file(&self.matches, &self.accounts, &command),
        }

        // apply(&self.matches, &self.accounts)
        //Ok(())
    }
}
