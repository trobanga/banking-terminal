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

    actions::perform_action(&matches, &file)?;
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
                .value_name("CONFIG_FILE")
                .help("Sets a custom config file"),
        )
        .subcommand(SubCommand::with_name("show"))
        .subcommand(
            SubCommand::with_name("get")
                .about("add amount of m0ney to category")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("from").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("borrow")
                .about("borrow m0ney to some friend in need")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("to").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("spend")
                .about("spent m0ney on")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("on").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("repay")
                .about("friend repayed m0ney")
                .arg(Arg::with_name("amount").required(true).index(1))
                .arg(Arg::with_name("back").required(true).index(2)),
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

fn init(config: &str) -> HashMap<String, String> {
    let config = Ini::load_from_file(config).unwrap();
    config.section(Some("Files")).unwrap().to_owned()
}
