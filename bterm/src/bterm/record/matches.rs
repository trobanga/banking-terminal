use clap::ArgMatches;
use std::collections::HashMap;
use std::error::Error;

pub mod actions;
pub use actions::Actions;


pub enum Commands {
    Accounts,
    Show,
    Get,
    Spend,
    Borrow,
    Repay,
}

impl Commands {
    pub fn from_matches(matches: &Matches) -> Option<Self> {
        match matches.command {
            Some("accounts") => Some(Commands::Accounts),
            Some("show") => Some(Commands::Show),
            Some("get") => Some(Commands::Get),
            Some("spend") => Some(Commands::Spend),
            Some("borrow") => Some(Commands::Borrow),
            Some("repay") => Some(Commands::Repay),
            _ => None,
        }
    }

    pub fn name_string(&self) -> String {
        match *self {
            Commands::Accounts => String::from("accounts"),
            Commands::Show => String::from("show"),
            Commands::Get => String::from("get"),
            Commands::Spend => String::from("spend"),
            Commands::Borrow => String::from("borrow"),
            Commands::Repay => String::from("repay"),
        }
    }

    pub fn sign(&self) -> Result<f32, Box<dyn Error>> {
        match *self {
            Commands::Get => Ok(1.),
            Commands::Spend => Ok(-1.),
            Commands::Borrow => Ok(-1.),
            Commands::Repay => Ok(1.),
            _ => Err("This action has no sign".into()),
        }
    }
}

pub struct Matches {
    config_file: String,
    command: Command,
    subcommand: HashMap<String, String>    
}

pub fn parse_matches(matches: &ArgMatches) -> Result<Matches, Box<dyn Error>> {
    let config_file = config_file(&matches);
    let command = match matches.subcommand_name() {
        Some(name) => name.to_owned(),
        None => return Err("No command given".into())
    };

    let mut subcommand = HashMap::new();
    
    match matches.subcommand() {
        ("show", Some(m)) => {
            let account = m.value_of("account").unwrap_or("__ALL__").to_owned();
            subcommand.insert(String::from("account"), account);
        }
        ("accounts", Some(m)) => {

        }
        (cmd, Some(m)) => {
            for n in &["amount", "description", "account"] {
                let v = matches.value_of(n).unwrap().to_owned();
                subcommand.insert(String::from(*n), v);
            }
        }
        (_, None) => {}
    }
    
    Ok(Matches {config_file, command, subcommand})
}

fn config_file(matches: &ArgMatches) -> String {
    let mut path = home_dir();
    path.push_str("/.bterm/config");
    String::from(matches.value_of("config").unwrap_or(&path))
}

fn home_dir() -> String {
    String::from(dirs::home_dir().unwrap().to_str().unwrap())
}
