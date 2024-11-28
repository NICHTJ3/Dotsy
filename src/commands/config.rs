extern crate glob;
use crate::{configs::DotsyConfig, install_configs, uninstall_configs};
use glob::glob;

pub fn install(configs: Vec<String>, config: &DotsyConfig) {
    install_configs(configs, config)
}

pub fn uninstall(configs: Vec<String>, config: &DotsyConfig) {
    uninstall_configs(configs, config);
}

pub fn list(config: &DotsyConfig) {
    let configs_regex = &config
        .dotfiles
        .join(&config.configs_dir)
        .join("*.config.json");

    let configs = glob(configs_regex.to_str().unwrap()).expect("Failed to read glob pattern");

    println!("Available Configs to install");
    configs.for_each(|e| {
        println!(" - {}", e.unwrap().display());
    });
    println!();
}
