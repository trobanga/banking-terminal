use clap::ArgMatches;
use std::error::Error;

pub enum Actions {
    Accounts,
    Show,
    Get,
    Spend,
    Borrow,
    Repay,
}

impl Actions {
    pub fn from_matches(matches: &Matches) -> Option<Self> {
        match matches.subcommand_name() {
            Some("accounts") => Some(Actions::Accounts),
            Some("show") => Some(Actions::Show),
            Some("get") => Some(Actions::Get),
            Some("spend") => Some(Actions::Spend),
            Some("borrow") => Some(Actions::Borrow),
            Some("repay") => Some(Actions::Repay),
            _ => None,
        }
    }

    pub fn name_string(&self) -> String {
        match *self {
            Actions::Accounts => String::from("accounts"),
            Actions::Show => String::from("show"),
            Actions::Get => String::from("get"),
            Actions::Spend => String::from("spend"),
            Actions::Borrow => String::from("borrow"),
            Actions::Repay => String::from("repay"),
        }
    }

    pub fn sign(&self) -> Result<f32, Box<dyn Error>> {
        match *self {
            Actions::Get => Ok(1.),
            Actions::Spend => Ok(-1.),
            Actions::Borrow => Ok(-1.),
            Actions::Repay => Ok(1.),
            _ => Err("This action has no sign".into()),
        }
    }
}