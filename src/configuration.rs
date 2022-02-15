use std::fs;

use lazy_static::lazy_static;


pub struct Configuration {
    token: String
}
impl Configuration {
    pub fn new(token: String) -> Configuration {
        Configuration {
            token
        }
    }

    pub fn from_config() -> Configuration {
        let token = fs::read_to_string("config")
            .expect("Failed to read config; does it exist?");
        Configuration::new(token)
    }

    pub fn get_token(&self) -> String {
        (&self.token).to_string()
    }
}

lazy_static! {
    pub static ref CONFIGURATION: Configuration = Configuration::from_config();
}
