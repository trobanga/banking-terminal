use std::collections::HashMap;
use std::error::Error;

pub type Accounts = HashMap<String, String>;

pub fn new() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn list(accounts: &Accounts) -> Result<(), Box<dyn Error>> {
    for account in accounts.keys() {
        println!("{}", account);
    }
    Ok(())
}
