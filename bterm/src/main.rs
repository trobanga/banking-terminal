use clap::{App, Arg, ArgMatches, SubCommand};
use dirs;
use yaml_rust::yaml;

use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

mod record;
use record::Account;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_config();
    let config = config_file(&matches);
    let accounts = init(&config);
    record::apply(&matches, &accounts)?;
    Ok(())
}

fn parse_config<'a>() -> ArgMatches<'a> {
    App::new("bterm")
        .version("0.1")
        .author("Daniel Hahne")
        .about("keep track of your m0ney")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config_file")
                .help("Sets a custom config file"),
        )
        .subcommand(
            SubCommand::with_name("account")
                .arg(
                    Arg::with_name("list_accounts")
                        .short("l")
                        .long("list-accounts"),
                )
                .arg(Arg::with_name("new").short("n")),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("show account content")
                .arg(Arg::with_name("account").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("add amount of m0ney to category")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("description").required(true).index(2))
                .arg(Arg::with_name("account").required(true).index(3)),
        )
        .subcommand(
            SubCommand::with_name("borrow")
                .about("borrow m0ney to some friend in need")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("description").required(true).index(2))
                .arg(Arg::with_name("account").required(true).index(3)),
        )
        .subcommand(
            SubCommand::with_name("spend")
                .about("spent m0ney on")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("description").required(true).index(2))
                .arg(Arg::with_name("account").required(true).index(3)),
        )
        .subcommand(
            SubCommand::with_name("repay")
                .about("friend repayed m0ney")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("description").required(true).index(2))
                .arg(Arg::with_name("account").required(true).index(3)),
        )
        .get_matches()
}

fn config_file(matches: &ArgMatches) -> String {
    let mut path = home_dir();
    path.push_str("/.bterm/config");
    String::from(matches.value_of("config").unwrap_or(&path))
}

fn home_dir() -> String {
    String::from(dirs::home_dir().unwrap().to_str().unwrap())
}

fn init(config: &str) -> Account {
    let mut f = File::open(&(config.to_owned() + ".yaml")).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let docs = yaml::YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    let mut accounts = Account::new();
    if let yaml::Yaml::Hash(ref h) = doc["Accounts"] {
        for (k, v) in h.iter() {
            let name = k.as_str().unwrap();
            let file = v["file"].as_str().unwrap();
            accounts.insert(String::from(name), String::from(file));
        }
    }
    accounts
}
