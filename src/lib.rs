pub mod cli;
pub mod configs;
pub mod defaults;
pub mod error;
pub mod handlers;
pub mod macros;

use std::path::PathBuf;

use configs::{ConfigFile, DotsyConfig};
use error::DotsyError;
extern crate shellexpand;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

// FIXME: This stuff should be handled better (there is a lot of duplicate logic

fn absolute(base: PathBuf) -> PathBuf {
    match shellexpand::tilde(&base.into_os_string().to_str().unwrap()) {
        std::borrow::Cow::Borrowed(s) => PathBuf::from(s),
        std::borrow::Cow::Owned(s) => PathBuf::from(s),
    }
}

pub fn install_configs(configs: Vec<String>, global_config: &DotsyConfig) {
    for config in configs {
        install_config(config, global_config)
    }
}

pub fn uninstall_configs(configs: Vec<String>, global_config: &DotsyConfig) {
    for config in configs {
        uninstall_config(config, global_config);
    }
}

fn uninstall_config(config: String, global_config: &DotsyConfig) {
    println!("Attempting to uninstall config: {}", config);

    let config = {
        let this = configs::ConfigConfig::load_by_name(&config, &global_config);
        match this {
            Ok(t) => t,
            Err(e) => return eprintln!("{}", e),
        }
    };

    // Unlink files
    for link in config.links.unwrap_or_default() {
        handlers::link::unlink_file(&absolute(PathBuf::from(link.to)))
            .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run cleanup scripts
    for script in config.revert_shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }
}

fn install_config(config: String, global_config: &DotsyConfig) {
    println!("Attempting to install config: {}", config);

    let config = {
        let this = configs::ConfigConfig::load_by_name(&config, &global_config);
        match this {
            Ok(t) => t,
            Err(e) => return eprintln!("{}", e),
        }
    };

    // TODO: Extract this logic
    // Link files
    // TODO: I need to work more on paths logic
    for link in config.links.unwrap_or_default() {
        handlers::link::link_file(
            absolute(
                global_config
                    .dotfiles
                    .join(&global_config.configs_dir)
                    .join(link.from),
            ),
            absolute(link.to),
            true,
        )
        .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run scripts
    for script in config.shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Make directories
    for dir in config.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| eprintln!("{}", e));
    }
}

pub fn install_profiles(profiles: Vec<String>, global_config: &DotsyConfig) {
    for profile in profiles {
        install_profile(profile, global_config)
    }
}

pub fn uninstall_profiles(profiles: Vec<String>, global_config: &DotsyConfig) {
    for profile in profiles {
        uninstall_profile(profile, global_config);
    }
}

fn uninstall_profile(profile: String, global_config: &DotsyConfig) {
    println!("Attempting to uninstall profile: {}", profile);
    let profile = configs::ProfileConfig::load_by_name(&profile, global_config).unwrap();

    // Unlink files
    for link in profile.links.unwrap_or_default() {
        handlers::link::unlink_file(&absolute(PathBuf::from(link.from)))
            .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run cleanup scripts
    for script in profile.revert_shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Uninstall configs
    for config in profile.configs.unwrap_or_default() {
        uninstall_config(config, global_config);
    }
}

fn install_profile(profile: String, global_config: &DotsyConfig) {
    println!("Attempting to install profile: {}", profile);
    let profile = configs::ProfileConfig::load_by_name(&profile, global_config).unwrap();

    // TODO: Extract this logic
    // Link files
    for link in profile.links.unwrap_or_default() {
        handlers::link::link_file(
            absolute(PathBuf::from(link.from)),
            absolute(PathBuf::from(link.to)),
            true,
        )
        .unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Make directories
    for dir in profile.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Run scripts
    for script in profile.shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| eprintln!("{}", e));
    }

    // Install configs
    install_configs(profile.configs.unwrap_or_default(), global_config);
}

// TODO: Find a way to cache the load of the rcfile for the life of the program
pub fn load_rcfile() -> DotsyResult<DotsyConfig> {
    let rcfile_path = defaults::fallback_path().unwrap();

    let mut config = configs::DotsyConfig::load(rcfile_path).unwrap();
    config.dotfiles = absolute(config.dotfiles);

    return Ok(config);
}
