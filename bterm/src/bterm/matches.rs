use clap::ArgMatches;
use std::collections::HashMap;
use std::error::Error;

mod commands;
pub use commands::Commands;

pub struct Matches {
    pub config_file: String,
    pub command: Commands,
    pub subcommands: HashMap<String, String>,
}

impl Matches {
    pub fn get(&self, n: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.subcommands.get(n).unwrap().to_owned())
    }
}

pub fn parse_matches(matches: &ArgMatches) -> Result<Matches, Box<dyn Error>> {
    let config_file = config_file();
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
        ("accounts", Some(m)) => {
            if m.is_present("list") {
                subcommands.insert(String::from("list"), "".to_owned());
            } else if let Some(x) = m.value_of("new") {
                subcommands.insert(String::from("new"), x.to_owned());
            } else if let Some(x) = m.value_of("delete") {
                subcommands.insert(String::from("delete"), x.to_owned());
            }
        }
        (_cmd, Some(m)) => {
            for n in &["amount", "description", "account"] {
                let v = m.value_of(n).unwrap().to_owned();
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

fn config_file() -> String {
    let mut path = home_dir();
    path.push_str("/.bterm/config.json");
    String::from(&path)
}

fn home_dir() -> String {
    String::from(dirs::home_dir().unwrap().to_str().unwrap())
}
