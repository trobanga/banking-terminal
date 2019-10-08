use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

mod bterm;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_config();
    let bt = bterm::BTerm::new(&matches);
    bt.apply()
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
        .subcommand(
            SubCommand::with_name("accounts")
                .arg(Arg::with_name("list").short("l").long("list"))
                .arg(
                    Arg::with_name("new")
                        .short("n")
                        .long("new")
                        .takes_value(true)
                        .value_name("ACCOUNT_NAME"),
                )
                .arg(
                    Arg::with_name("delete")
                        .short("d")
                        .long("rm")
                        .takes_value(true)
                        .value_name("ACCOUNT_NAME"),
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("show account content")
                .arg(Arg::with_name("account").index(1)),
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
