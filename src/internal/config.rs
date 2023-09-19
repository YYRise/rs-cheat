use serde_yaml::from_str as yaml_from_str;
use std::fs::read_to_string;
use serde::{Deserialize, Serialize};
use schemars::schema::RootSchema;
use serde_json::{from_str as json_from_str, to_string_pretty};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub colorize: bool,
    pub editor: String,
    // pub cheatpaths: Vec<Cheatpath>,
    pub style: String,
    pub formatter: String,
    pub pager: String,
    pub path: String,
    pub cheatpath: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Cheatpath {
    pub name: String,
    pub path: String,
    pub read_only: bool,
    pub tags: Vec<String>,
}

impl Config {
    pub fn new(conf_path: &str, resolve: bool) -> Option<Config> {
        let schema = yaml_from_str::<RootSchema>(
            &read_to_string(conf_path).expect(format!("Error loading configuration file {}", conf_path).as_str()),
        );
        println!("{:#?}", schema);
        match schema {
            Ok(json) => {
                let data = to_string_pretty(&json).unwrap_or_else(|_| panic!("{} file data error！, please check the configuration!", conf_path));
                println!("data = {:#?}", &data);
                let p :Config = json_from_str(&*data).expect("Failed to transfer JSON data to Config object！");
                return Some(p);
            }
            Err(err) => {
                println!("{}", err);
                return None;
            }
        };
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config() {
        let conf_path = String::from("conf/conf.yml");
        let conf = Config::new(conf_path.as_str(), false);
        println!("{:#?}", conf);
        // assert_eq!(conf.colorize, true);
        conf.as_ref().map(|a| {
            println!("Config:{}", serde_json::to_string(&a).unwrap());
        });
    }
}