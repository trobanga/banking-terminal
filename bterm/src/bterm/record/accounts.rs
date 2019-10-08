use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;

#[derive(Debug)]
pub struct Accounts {
    pub accounts: HashMap<String, String>,
    config_file: String,
}

impl Accounts {
    pub fn new(config_file: &str) -> Self {
        let accounts = match Accounts::read_config(&config_file) {
            Ok(x) => x,
            _ => panic!("AAA"),
        };
        let config_file = config_file.to_owned();
        Self {
            accounts,
            config_file,
        }
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        for account in self.accounts.keys() {
            println!("{}", account);
        }
        Ok(())
    }

    fn read_config(config: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut f = File::open(&(config.to_owned() + ".yaml"))?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let docs = yaml::YamlLoader::load_from_str(&contents)?;
        let doc = &docs[0];

        let mut accounts = HashMap::new();
        if let yaml::Yaml::Hash(ref h) = doc["Accounts"] {
            for (k, v) in h.iter() {
                let name = k.as_str().unwrap();
                let file = v["file"].as_str().unwrap();
                accounts.insert(String::from(name), String::from(file));
            }
        }
        Ok(accounts)
    }

    pub fn new_account(&self, _config: &str, name: &str) -> Result<(), Box<dyn Error>> {
        if self.accounts.contains_key(name) {
            return Err("An account with that name already exists.".into());
        }

        Ok(())
    }
}
