extern crate serde_json;
use self::serde_json::Value;

pub struct Config();

impl Config {
    // add code here
    pub fn read_as_json(config_file : String) -> Value {
        let json_config: Value = serde_json::from_str(&config_file).expect("Cannot read json config file");
        json_config
    }
}