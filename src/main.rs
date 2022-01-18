mod cli;
mod commands;
mod config;

use crate::cli::CLI;
use crate::config::Config;
use clap::Parser;
use dirs::home_dir;
use std::path::Path;

fn init() -> String {
    let home_dir = home_dir().unwrap().display().to_string();
    let target_dir = format!("{home_dir}/.project_builder/").to_string();
    if !Path::new(&target_dir).exists() {
        let res = std::fs::create_dir(&target_dir);
        if res.is_ok() {
            println!("Created directory to store configs");
        } else {
            println!("Failed to create directory");
        }
    }
    return target_dir;
}

fn handle_command(command: &str, config_path: &str) {
    if command == "build" {
        commands::execute_build(&config_path);
    }
    if command == "set_command" {
        commands::set_build_command(&config_path);
    }
    if command == "set_dir" {
        commands::set_build_dir(&config_path);
    }
    if command == "set_args" {
        commands::set_build_args(&config_path);
    }
    if command == "set_flags" {
        commands::set_build_flags(&config_path);
    }
}

fn main() {
    let target_dir = init();
    let args = CLI::parse();
    let configs_path = Path::new(&target_dir);
    let project_name = args.project_name;
    let config_exists = configs_path.join(format!("{project_name}.json")).exists();
    let config_path = configs_path
        .join(format!("{project_name}.json"))
        .display()
        .to_string();
    let command = args.subcommand;
    if config_exists {
        handle_command(&command, &config_path);
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
