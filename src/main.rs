mod cli;
mod commands;
mod config;

use crate::cli::CLI;
use crate::config::Config;
use std::path::Path;
use clap::Parser;

fn init() {
    if !Path::new("/home/scott/.project_builder/").exists() {
        let res = std::fs::create_dir("/home/scott/.project_builder/");
        if res.is_ok() {
            println!("Created directory to store configs");
        } else {
            println!("Failed to create directory");
        }
    }
}

fn main() {
    init();
    let args = CLI::parse();
    let configs_path = Path::new("/home/scott/.project_builder/");
    let project_name = args.project_name;
    let config_exists = configs_path.join(format!("{project_name}.json")).exists();
    let config_path = configs_path.join(format!("{project_name}.json")).display().to_string();
    if config_exists {
        commands::execute_build(&config_path);
        println!("Hello, world!");
    } else {
        println!("No config file found, would you like to create one? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            let result = Config::init_file(&config_path, &project_name);
            if result.is_ok() {
                println!("Config file created for project {project_name}");
            } else {
                println!("Config file creation failed");
            }
        } else {
            println!("Exiting...");
        }
    }
}
