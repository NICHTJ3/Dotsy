//! # Dotsy - A Modular Dotfile Manager
//!
//! Dotsy is a flexible dotfile manager that helps you manage configurations,
//! profiles, and system setup through a declarative configuration system.
//!
//! ## Architecture
//!
//! The codebase is organized into several key modules:
//!
//! - **configs**: Configuration file structures and management
//! - **handlers**: File, package, and script operation handlers
//! - **profiles**: Profile installation and management
//! - **utils**: Shared utilities including file management, logging, and builders
//! - **plugins**: Extensible plugin system for custom functionality
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use dotsy::{load_rcfile, DotsyResult};
//!
//! fn main() -> DotsyResult<()> {
//!     let config = load_rcfile()?;
//!     // Use the configuration...
//!     Ok(())
//! }
//! ```
//!
//! ## Thread Safety
//!
//! The current implementation is primarily single-threaded. The plugin system
//! is designed with `Send + Sync` bounds to support future concurrent execution.
//!
//! ## Error Handling
//!
//! All operations return `DotsyResult<T>` which uses the Snafu error library
//! for comprehensive, context-rich error messages.

pub mod cli;
pub mod commands;
pub mod configs;
pub mod defaults;
pub mod error;
pub mod handlers;
pub mod macros;
pub mod plugins;
pub mod profiles;
pub mod utils;

use ansi_term::Colour::Green;

use configs::{ConfigFile, DotsyConfig, Link};
use error::DotsyError;
use utils::path::absolute;
extern crate shellexpand;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

// FIXME: This stuff should be handled better (there is a lot of duplicate logic)

pub fn get_absolute_link(link: Link, global_config: &DotsyConfig) -> Link {
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

    // Uninstall packages
    for package in config.packages.unwrap_or_default() {
        handlers::package::uninstall_package(&package, &global_config.package_remove_command)
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

    // Make directories
    for dir in config.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Install packages
    for package in config.packages.unwrap_or_default() {
        handlers::package::install_package(&package, &global_config.package_add_command)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run scripts
    for script in config.shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }
}

fn uninstall_profile(_profile: String, global_config: &DotsyConfig) {
    profiles::profile_manager::uninstall(_profile, global_config);
}

fn install_profile(_profile: String, global_config: &DotsyConfig) {
    profiles::profile_manager::install(_profile, global_config);
}

pub fn load_rcfile() -> DotsyResult<DotsyConfig> {
    let rcfile_path = defaults::fallback_path().unwrap();

    let mut config = configs::DotsyConfig::load(rcfile_path).unwrap();
    config.dotfiles = absolute(config.dotfiles);

    Ok(config)
}
