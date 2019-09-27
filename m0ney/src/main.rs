use dirs;

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

extern crate ini;
use ini::Ini;

use std::collections::HashMap;
use std::error::Error;

pub mod actions;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_config();
    let config = config_file(&matches);

    let files = init(&config);
    let file = &files["m0ney_file"];
    
    actions::perform_action(&matches, &file)
}

fn parse_config<'a>() -> ArgMatches<'a> {
    App::new("m0ney")
        .version("0.1")
        .author("Daniel Hahne")
        .about("keep track of your m0ney")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Sets a custom config file"),
        )
        .arg(Arg::with_name("show").short("s").long("show"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add amount of money to category")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("category").required(true).index(2)),
        )
        .get_matches()
}

fn config_file(matches: &ArgMatches) -> String {
    let mut path = home_dir();
    path.push_str("/.m0ney/config");
    String::from(matches.value_of("config").unwrap_or(&path))
}

fn home_dir() -> String {
    String::from(dirs::home_dir().unwrap().to_str().unwrap())
}

fn init(config: &str) -> HashMap<String, String> {
    let config = Ini::load_from_file(config).unwrap();
    config.section(Some("Files")).unwrap().to_owned()
}

