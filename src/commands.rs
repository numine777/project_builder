use tokio::process::Command;
use crate::config::Config;
use std::path::Path;
// use std::error::Error;
// use std::process::Output;

#[tokio::main]
pub async fn execute_command(config_path: &str) {
    let config = Config::from_file(config_path).unwrap();
    let _command = Command::new(&config.build_command)
        .args(&config.args)
        .args(&config.flags)
        .current_dir(Path::new(&config.project_dir))
        .spawn();
}

pub fn set_command(config_path: &str) {
    let mut execution_command = String::new();
    println!("Enter a command to run");
    std::io::stdin().read_line(&mut execution_command).unwrap();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_execution_command(&execution_command.trim());
    config.to_file(config_path).unwrap();
}

pub fn set_execution_dir(config_path: &str) {
    let mut execution_dir = String::new();
    println!("Enter a execution command");
    std::io::stdin().read_line(&mut execution_dir).unwrap();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_dir(&execution_dir.trim());
    config.to_file(config_path).unwrap();
}

pub fn set_execution_args(config_path: &str) {
    let mut execution_args = String::new();
    println!("Enter a command argument");
    std::io::stdin().read_line(&mut execution_args).unwrap();
    let args = execution_args
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_args(args);
    config.to_file(config_path).unwrap();
}

pub fn set_execution_flags(config_path: &str) {
    let mut execution_flags = String::new();
    println!("Enter a execution argument");
    std::io::stdin().read_line(&mut execution_flags).unwrap();
    let args = execution_flags
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_flags(args);
    config.to_file(config_path).unwrap();
}

pub fn clear_flags(config_path: &str) {
    let mut config = Config::from_file(config_path).unwrap();
    config.clear_flags();
    config.to_file(config_path).unwrap();
}

pub fn clear_args(config_path: &str) {
    let mut config = Config::from_file(config_path).unwrap();
    config.clear_args();
    config.to_file(config_path).unwrap();
}

pub fn list_configs(config_path: &str) {
    let configs = Config::list_configs(config_path).unwrap();
    for config in configs {
        println!("{}", config);
    }
}

// Broken, fix this
// fn get_output(command: &str, dir: &str) -> Result<Output, Box<dyn Error>> {
//     let output = std::process::Command::new(command)
//         .current_dir(Path::new(dir))
//         .output()
//         .unwrap();
//     return Ok(output);
// }
//
// pub fn search_configs(config_path: &str) -> String {
//     let output: Output = get_output("fzf", config_path).unwrap();
//     return String::from_utf8(output.stdout).unwrap();
// }
