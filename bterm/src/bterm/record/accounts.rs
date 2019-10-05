use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;

pub type Accounts = HashMap<String, String>;

pub fn list(accounts: &Accounts) -> Result<(), Box<dyn Error>> {
    for account in accounts.keys() {
        println!("{}", account);
    }
    Ok(())
}

pub fn init(config: &str) -> Result<Accounts, Box<dyn Error>> {
    let mut f = File::open(&(config.to_owned() + ".yaml"))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let docs = yaml::YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];

    let mut accounts = Accounts::new();
    if let yaml::Yaml::Hash(ref h) = doc["Accounts"] {
        for (k, v) in h.iter() {
            let name = k.as_str().unwrap();
            let file = v["file"].as_str().unwrap();
            accounts.insert(String::from(name), String::from(file));
        }
    }
    Ok(accounts)
}
