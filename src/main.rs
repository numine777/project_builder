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
    match command {
        "run" => {
            commands::execute_command(&config_path);
        }
        "set_command" => {
            commands::set_command(&config_path);
        }
        "set_dir" => {
            commands::set_execution_dir(&config_path);
        }
        "set_args" => {
            commands::set_execution_args(&config_path);
        }
        "clear_args" => {
            commands::clear_args(&config_path);
        }
        "set_flags" => {
            commands::set_execution_flags(&config_path);
        }
        "clear_flags" => {
            commands::clear_flags(&config_path);
        }
        "list" => {
            commands::list_configs(&config_path);
        }
        _default => {
            println!("Command not found");
        }
    }
}

fn create_command_config(config_path: &str, project_name: &str) {
    let result = Config::init_file(&config_path, &project_name);
    if result.is_ok() {
        println!("Config file created for project {project_name}");
    } else {
        println!("Config file creation failed");
    }
}

fn set_config_path(target_dir: &str, project_name: &str) -> String {
    let configs_path = Path::new(&target_dir);
    if project_name == "" {
        return configs_path.display().to_string();
    }
    let config_exists = configs_path.join(format!("{project_name}.json")).exists();
    let config_path = configs_path
        .join(format!("{project_name}.json"))
        .display()
        .to_string();
    if !config_exists {
        println!("No config file found, would you like to create one? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            create_command_config(&config_path, &project_name);
        } else {
            println!("Exiting...");
        }
    }
    return config_path;
}

fn main() {
    let target_dir = init();
    let args = CLI::parse();
    let command = args.command;
    let config_path = set_config_path(&target_dir, &args.project_name);
    if command != "init" {
        handle_command(&command, &config_path);
    }
    // TODO(Scott): Add back in when function works.
    // if project_name == "search" {
    //     project_name = commands::search_configs(&configs_path.display().to_string());
    // }
}
