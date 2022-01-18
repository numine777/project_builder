use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_name: String,
    pub project_dir: String,
    pub build_command: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut json_str = String::new();
        buf_reader.read_to_string(&mut json_str)?;
        let config: Config = serde_json::from_str(&json_str)?;
        Ok(config)
    }
    pub fn to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let json_str = serde_json::to_string_pretty(self)?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
    pub fn init_file(path: &str, project_name: &str) -> Result<(), std::io::Error> {
        let config = Config {
            project_name: format!("{project_name}"),
            project_dir: "".to_string(),
            build_command: "".to_string(),
            args: vec![],
            flags: vec![],
        };
        config.to_file(path)
    }
    pub fn update_build_command(&mut self, command: &str) {
        self.build_command = command.to_string();
    }
    pub fn add_args(&mut self, mut args: Vec<String>) {
        self.args.append(&mut args);
    }
    pub fn add_flags(&mut self, mut flags: Vec<String>) {
        self.flags.append(&mut flags);
    }
}
