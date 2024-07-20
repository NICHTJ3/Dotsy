pub mod cli;
pub mod commands;
pub mod configs;
pub mod defaults;
pub mod error;
pub mod handlers;
pub mod macros;

use ansi_term::Colour::Green;
use std::path::PathBuf;

use std::fs;

use configs::{ConfigFile, DotsyConfig, Link};
use error::DotsyError;
extern crate shellexpand;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

// FIXME: This stuff should be handled better (there is a lot of duplicate logic)

fn get_absolute_link(link: Link, global_config: &DotsyConfig) -> Link {
    let from = absolute(
        global_config
            .dotfiles
            .join(&global_config.configs_dir)
            .join(link.from),
    );
    let to = absolute(link.to);

    Link {
        from,
        to,
        glob: link.glob,
    }
}

fn link_exists(path: &PathBuf) -> bool {
    let metadata = fs::symlink_metadata(path);
    if metadata.is_err() {
        return false;
    }
    true
}

fn is_symlink(path: &PathBuf) -> bool {
    let metadata = fs::symlink_metadata(path);
    if metadata.is_err() {
        return false;
    }
    metadata.unwrap().file_type().is_symlink()
}

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
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to uninstall config"),
        arg = config
    );

    let config = {
        let this = configs::ConfigConfig::load_by_name(&config, global_config);
        match this {
            Ok(t) => t,
            Err(e) => return dotsy_log_error!("{}", e),
        }
    };

    // Unlink files
    for link in config.links.unwrap_or_default() {
        handlers::link::unlink_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run cleanup scripts
    for script in config.revert_shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }
}

fn install_config(config: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to install config"),
        arg = config
    );

    let config = {
        let this = configs::ConfigConfig::load_by_name(&config, global_config);
        match this {
            Ok(t) => t,
            Err(e) => return dotsy_log_error!("{}", e),
        }
    };

    // TODO: Extract this logic
    // Link files
    // TODO: I need to work more on paths logic
    for link in config.links.unwrap_or_default() {
        handlers::link::link_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run scripts
    for script in config.shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Make directories
    for dir in config.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }
}

fn uninstall_profile(_profile: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to uninstall profile"),
        arg = _profile
    );
    let profile = configs::ProfileConfig::load_by_name(&_profile, global_config).unwrap();

    // Unlink files
    for link in profile.links.unwrap_or_default() {
        handlers::link::unlink_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run cleanup scripts
    for script in profile.revert_shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Uninstall configs
    for config in profile.configs.unwrap_or_default() {
        uninstall_config(config, global_config);
    }
    println!(
        "{message}: {arg}",
        message = Green.paint("Finished uninstalling profile"),
        arg = _profile
    );
}

fn install_profile(_profile: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to install profile"),
        arg = _profile
    );
    let profile = configs::ProfileConfig::load_by_name(&_profile, global_config).unwrap();

    // TODO: Extract this logic
    // Link files
    for link in profile.links.unwrap_or_default() {
        handlers::link::link_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Make directories
    for dir in profile.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run scripts
    for script in profile.shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Install configs
    install_configs(profile.configs.unwrap_or_default(), global_config);

    println!(
        "{message}: {arg}",
        message = Green.paint("Finished installing profile"),
        arg = _profile
    );
}

// TODO: Find a way to cache the load of the rcfile for the life of the program
pub fn load_rcfile() -> DotsyResult<DotsyConfig> {
    let rcfile_path = defaults::fallback_path().unwrap();

    let mut config = configs::DotsyConfig::load(rcfile_path).unwrap();
    config.dotfiles = absolute(config.dotfiles);

    Ok(config)
}
