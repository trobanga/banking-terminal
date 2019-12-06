use clap::ArgMatches;
use std::error::Error;

mod record;
use record::Matches;
use record::accounts::Accounts;
use record::{parse_matches, apply};

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

    pub fn apply(&self) -> Result<(), Box<dyn Error>> {
        apply(&self.matches, &self.accounts)
    }
}
