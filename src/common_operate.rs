use std::fs;
use toml::Value;


#[derive(Debug,PartialEq, Eq)]
pub struct Address {
    pub ip: String,
    pub port: u16,
    pub access_ip: String,
    pub access_port: u16
}

impl Address {
    pub fn new(ip: String, port: String, access_ip: String, access_port: String) -> Self {
        let port: u16 = port.parse().expect("Wrong type of port number.");
        let access_port: u16 = access_port.parse().expect("Wrong type of access_port number.");
        Address { ip , port , access_ip, access_port}
    }
}

pub fn get_address() -> Address {
    let config_content: String = fs::read_to_string("config/app.toml").expect("Cannot read app.toml.");
    let config: Value = toml::from_str(&config_content).expect("Unable to parse configuration file.");
    let ip = config["address"]["ip"].as_str().expect("Unable to read IP address information.").to_owned();
    let port = config["address"]["port"].as_str().expect("Unable to parse port information.").to_owned();
    let access_ip = config["address"]["access_ip"].as_str().expect("Unable to parse access information.").to_owned();
    let access_port = config["address"]["access_port"].as_str().expect("Unable to parse access information.").to_owned();
    Address::new(ip, port, access_ip, access_port)
}

