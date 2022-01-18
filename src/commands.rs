use crate::config::Config;

pub fn execute_build(config_path: &str) {
    let config = Config::from_file(config_path).unwrap();
    let command = std::process::Command::new(&config.build_command)
        .args(&config.args)
        .args(&config.flags)
        .spawn();
}
