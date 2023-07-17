use std::fs;
use toml::Value;


#[derive(Debug,PartialEq, Eq)]
pub struct Address {
    pub ip: String,
    pub port: u16,
    pub access: String
}

impl Address {
    pub fn new(ip: String, port: String, access: String) -> Self {
        let port: u16 = port.parse().expect("Wrong type of port number.");
        Address { ip , port , access}
    }
}

pub fn get_address() -> Address {
    let config_content: String = fs::read_to_string("config/app.toml").expect("Cannot read app.toml.");
    let config: Value = toml::from_str(&config_content).expect("Unable to parse configuration file.");
    let ip = config["address"]["ip"].as_str().expect("Unable to read IP address information.").to_owned();
    let port = config["address"]["port"].as_str().expect("Unable to parse port information.").to_owned();
    let access = config["address"]["access"].as_str().expect("Unable to parse access information.").to_owned();
    Address::new(ip, port, access)
}

