use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        let accounts: Value = serde_json::from_str(
            r#"
            {"accounts": [
              {"name": "GLS", "file": "/home/trobanga/.bterm/gls.csv"}, 
              {"name": "Cash", "file": "/home/trobanga/.bterm/cash.csv"}
            ]}         
     "#,
        )
        .unwrap();
        println!("{:?}", accounts);
        let x: Accounts = serde_json::from_value(accounts).unwrap();

        println!("{:?}", x);
        //        assert_eq!(accounts.name, String::from("GLS"));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    name: String,
    file: String,
}

impl Account {
    fn new(name: &str, file: &str) -> Self {
        Self {
            name: name.to_owned(),
            file: file.to_owned(),
        }
    }
}

impl Accounts {
    pub fn new(config_file: &str) -> Self {
        let contents = Accounts::read_from_config(&config_file).unwrap();
        let v: Value = serde_json::from_str(&contents).unwrap();
        let acc: Accounts = serde_json::from_value(v).unwrap();
        acc
    }

    fn read_from_config(config: &str) -> Result<(String), Box<dyn Error>> {
        let mut f = File::open(&config)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write_to_config(&self, config: &str) -> Result<(), Box<dyn Error>> {
        let contents = Accounts::read_from_config(&config)?;
        let mut v: Value = serde_json::from_str(&contents)?;

        let j = serde_json::to_value(&self.accounts)?;
        v["accounts"] = j;

        let mut f = File::create(&config)?;
        f.write_all(v.to_string().as_bytes())?;
        Ok(())
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        for account in &self.accounts {
            println!("{} : {}", account.name, account.file);
        }
        Ok(())
    }

    pub fn add_account(&mut self, config: &str, name: &str) -> Result<(), Box<dyn Error>> {
        for account in &self.accounts {
            if account.name == name {
                return Err("An account with that name already exists.".into());
            }
        }
        let mut f = String::from(dirs::home_dir().unwrap().to_str().unwrap());
        f.push_str("/.bterm/");
        f.push_str(&name);
        f.push_str(".csv");

        let account = Account::new(&name, &f);
        self.accounts.push(account);
        self.write_to_config(config)
    }
    
}
