use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use serde_json::{Value};
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        let accounts: Value = serde_json::from_str(r#"
            {"accounts": [
              {"name": "GLS", "file": "/home/trobanga/.bterm/gls.csv"}, 
              {"name": "Cash", "file": "/home/trobanga/.bterm/cash.csv"}
            ]}         
     "#).unwrap();
        println!("{:?}", accounts);
        let x: Accounts = serde_json::from_value(accounts).unwrap();

        println!("{:?}", x);
//        assert_eq!(accounts.name, String::from("GLS"));

        
    }    
}

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    name: String,
    file: String,
}

impl Account {
    fn new(name: &str, file: &str) -> Self {
        Self{name: name.to_owned(), file: file.to_owned()}
    }
}

impl Accounts {
    pub fn new(config_file: &str) -> Self {
        let accounts = Accounts::read_config(&config_file).unwrap();
        accounts
        // let config_file = config_file.to_owned();
        // Self {
        //     accounts// ,
        //     // config_file,
        // }
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        // for account in self.accounts() {
        //     println!("{}", account.file);
        // }
        Ok(())
    }

    fn read_config(config: &str) -> Result<(Accounts), Box<dyn Error>> {
        let mut f = File::open(&(config.to_owned()))?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        // let accounts: Accounts = toml::from_str(&contents).unwrap();
        let v: Value = serde_json::from_str(&contents)?;
        let acc: Accounts = serde_json::from_value(v).unwrap();
        println!("{:?}", acc);
        Ok(acc)
    }

    pub fn new_account(&self, _config: &str, name: &str) -> Result<(), Box<dyn Error>> {
        // if self.accounts.contains_key(name) {
        //     return Err("An account with that name already exists.".into());
        // }
        Ok(())
    }
}
