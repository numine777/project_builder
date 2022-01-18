use tokio::process::Command;
use crate::config::Config;
use std::error::Error;
use std::path::Path;
use std::process::Output;

#[tokio::main]
pub async fn execute_build(config_path: &str) {
    let config = Config::from_file(config_path).unwrap();
    let _command = Command::new(&config.build_command)
        .args(&config.args)
        .args(&config.flags)
        .current_dir(Path::new(&config.project_dir))
        .spawn();
}

pub fn set_build_command(config_path: &str) {
    let mut build_command = String::new();
    println!("Enter a build command");
    std::io::stdin().read_line(&mut build_command).unwrap();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_build_command(&build_command.trim());
    config.to_file(config_path).unwrap();
}

pub fn set_build_dir(config_path: &str) {
    let mut build_dir = String::new();
    println!("Enter a build command");
    std::io::stdin().read_line(&mut build_dir).unwrap();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_dir(&build_dir.trim());
    config.to_file(config_path).unwrap();
}

pub fn set_build_args(config_path: &str) {
    let mut build_args = String::new();
    println!("Enter a build argument");
    std::io::stdin().read_line(&mut build_args).unwrap();
    let args = build_args
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut config = Config::from_file(config_path).unwrap();
    config.set_args(args);
    config.to_file(config_path).unwrap();
}

pub fn set_build_flags(config_path: &str) {
    let mut build_flags = String::new();
    println!("Enter a build argument");
    std::io::stdin().read_line(&mut build_flags).unwrap();
    let args = build_flags
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
