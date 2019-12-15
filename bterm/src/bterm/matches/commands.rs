use std::error::Error;

#[derive(Debug)]
pub enum Commands {
    Accounts,
    Show,
    Get,
    Spend,
    Borrow,
    Repay,
}

impl Commands {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "accounts" => Some(Commands::Accounts),
            "show" => Some(Commands::Show),
            "get" => Some(Commands::Get),
            "spend" => Some(Commands::Spend),
            "borrow" => Some(Commands::Borrow),
            "repay" => Some(Commands::Repay),
            _ => None,
        }
    }

    pub fn name_string(&self) -> String {
        match *self {
            Commands::Accounts => String::from("accounts"),
            Commands::Show => String::from("show"),
            Commands::Get => String::from("get"),
            Commands::Spend => String::from("spend"),
            Commands::Borrow => String::from("borrow"),
            Commands::Repay => String::from("repay"),
        }
    }

    pub fn sign(&self) -> Result<f32, Box<dyn Error>> {
        match *self {
            Commands::Get => Ok(1.),
            Commands::Spend => Ok(-1.),
            Commands::Borrow => Ok(-1.),
            Commands::Repay => Ok(1.),
            _ => Err("This action has no sign".into()),
        }
    }
}
