use std::fs::File;
use std::io::Read;
use serde::Deserialize;

use crate::errors::{AppResult, Error};

#[derive(Debug, Deserialize)]
enum FieldType {
    String,
    Number
}

#[derive(Debug, Deserialize)]
pub struct Config {
    settings: Settings,
    models: Vec<Model>
}

#[derive(Debug, Deserialize)]
struct Settings {
    host: String,
    port: u16
}

#[derive(Debug, Deserialize)]
struct Model {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    data_type: FieldType
}

impl Config {
    pub fn from(file_path: &str) -> AppResult<Self> {
        let content = Self::open_file(file_path)?;
        let config: Self = Self::parse_config(&content)?;

        Ok(config)
    }

    fn open_file(file_path: &str) -> AppResult<String> {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut content = String::new();

                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        Ok(content)
                    },
                    Err(_) => Err(Error::OpenConfigFileFailed)
                }
            },
            Err(_) => Err(Error::OpenConfigFileFailed)
        }
    }

    fn parse_config(content: &str) -> AppResult<Self> {
        match serde_yaml::from_str(content) {
            Ok(config) => {
                Ok(config)
            },
            Err(_) => Err(Error::ParsingConfigFileFailed)
        }
    }
}