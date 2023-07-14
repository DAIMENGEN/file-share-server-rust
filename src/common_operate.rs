use std::fs;
use toml::Value;

pub struct Address {
    pub ip: String,
    pub port: u16,
    pub access: String
}

impl Address {
    fn new(ip: String, port: String, access: String) -> Self {
        let port: u16 = port.parse().expect("端口号类型错误");
        Address { ip , port , access}
    }
}

pub fn get_address() -> Address {
    let config_content: String = fs::read_to_string("config/app.toml").expect("cannot read app.toml");
    let config: Value = toml::from_str(&config_content).expect("无法解析配置文件");
    let ip = config["address"]["ip"].as_str().expect("无法读取IP地址信息").to_owned();
    let port = config["address"]["port"].as_str().expect("无法解析端口信息").to_owned();
    let access = config["address"]["access"].as_str().expect("无法解析访问信息").to_owned();
    Address::new(ip, port, access)
}

