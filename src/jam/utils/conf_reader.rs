use std::fs;
use yaml_rust::*;

pub struct Config {
    pub _src: String,
    pub config: yaml::Yaml,
}

impl Config {
    pub fn load(filepath: String) -> Config {
        Self {
            _src: match fs::read_to_string(filepath) {
                Ok(x) => x,
                Err(_) => {
                    std::eprintln!("Failed to load config file");
                    panic!("Bye!")
                }
            },
            config: Yaml::from_str(""),
        }
    }

    pub fn get_config(&mut self) -> Vec<Yaml> {
        match yaml::YamlLoader::load_from_str(&self._src) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Bye!")
            }
        }
    }
}
