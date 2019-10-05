use clap::ArgMatches;

pub mod record;
pub use record::apply;
pub use record::accounts::Accounts;

pub use record::parse_matches;

pub struct BTerm<'a> {
    config_file: String,
    matches: ArgMatches<'a>,
    accounts: Accounts
}

impl BTerm<'_> {
    // pub fn new<'a>(config_file: String,
    //            matches: ArgMatches<'_>,
    //            accounts: Accounts) -> Self {
    //     Self {config_file, matches, accounts }
    // }
}
