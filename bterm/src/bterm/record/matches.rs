use clap::ArgMatches;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum Commands {
    Accounts,
    Show,
    Get,
    Spend,
    Borrow,
    Repay,
}

impl Commands {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "accounts" => Some(Commands::Accounts),
            "show" => Some(Commands::Show),
            "get" => Some(Commands::Get),
            "spend" => Some(Commands::Spend),
            "borrow" => Some(Commands::Borrow),
            "repay" => Some(Commands::Repay),
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
    pub config_file: String,
    pub command: Commands,
    subcommands: HashMap<String, String>,
}

impl Matches {
    pub fn get(&self, n: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.subcommands.get(n).unwrap().to_owned())
    }
}

pub fn parse_matches(matches: &ArgMatches) -> Result<Matches, Box<dyn Error>> {
    let config_file = config_file(&matches);
    let command = match matches.subcommand_name() {
        Some(name) => Commands::from_name(&name).unwrap(),
        None => return Err("No command given".into()),
    };

    let mut subcommands = HashMap::new();

    match matches.subcommand() {
        ("show", Some(m)) => {
            let account = m.value_of("account").unwrap_or("__ALL__").to_owned();
            subcommands.insert(String::from("account"), account);
        }
        ("accounts", Some(_m)) => {}
        (_cmd, Some(_m)) => {
            for n in &["amount", "description", "account"] {
                let v = matches.value_of(n).unwrap().to_owned();
                subcommands.insert(String::from(*n), v);
            }
        }
        (_, None) => {}
    }

    Ok(Matches {
        config_file,
        command,
        subcommands,
    })
}

fn config_file(matches: &ArgMatches) -> String {
    let mut path = home_dir();
    path.push_str("/.bterm/config");
    String::from(matches.value_of("config").unwrap_or(&path))
}

fn home_dir() -> String {
    String::from(dirs::home_dir().unwrap().to_str().unwrap())
}
