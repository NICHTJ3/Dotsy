pub mod cli;
pub mod configs;
pub mod defaults;
pub mod error;
pub mod macros;
pub mod plugins;

use std::path::PathBuf;

use configs::{ConfigFile, DotsyConfig};
use error::DotsyError;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

// FIXME: This stuff should be handled better (there is a lot of duplicate logic

pub fn install_configs(configs: Vec<String>, global_config: &DotsyConfig) {
    for config in configs {
        install_config(config, global_config)
    }
}

fn install_config(config: String, _global_config: &DotsyConfig) {
    println!("Attempting to install config: {}", config);

    let config = {
        let this = configs::ConfigConfig::load_by_name(&config);
        match this {
            Ok(t) => t,
            Err(e) => return eprintln!("{}", e),
        }
    };

    // TODO: Extract this logic
    // Link files
    for link in config.links.unwrap_or_default() {
        plugins::link::link_file(PathBuf::from(link.from), PathBuf::from(link.to))
            .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run scripts
    for script in config.shell.unwrap_or_default() {
        plugins::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Make directories
    for dir in config.directories.unwrap_or_default() {
        plugins::files::create_dir(dir).unwrap_or_else(|e| eprintln!("{}", e));
    }
}

pub fn install_profiles(profiles: Vec<String>, global_config: &DotsyConfig) {
    for profile in profiles {
        install_profile(profile, global_config)
    }
}

fn install_profile(profile: String, global_config: &DotsyConfig) {
    println!("Attempting to install profile: {}", profile);
    let profile = configs::ProfileConfig::load_by_name(&profile).unwrap();

    // TODO: Extract this logic
    // Link files
    for link in profile.links.unwrap() {
        plugins::link::link_file(PathBuf::from(link.from), PathBuf::from(link.to))
            .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Make directories
    for dir in profile.directories.unwrap() {
        plugins::files::create_dir(dir).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run scripts
    for script in profile.shell.unwrap() {
        plugins::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Install configs
    install_configs(profile.configs.unwrap(), global_config);
}

// TODO: Find a way to cache the load of the rcfile for the life of the program
pub fn load_rcfile() -> DotsyConfig {
    let rcfile_path = defaults::fallback_path().unwrap();

    let config = configs::DotsyConfig::load(rcfile_path).unwrap();

    return config;
}
