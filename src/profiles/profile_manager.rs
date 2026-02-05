/// ProfileManager: Handles profile operations
use ansi_term::Colour::Green;

use crate::{
    configs::{DotsyConfig, ProfileConfig},
    dotsy_log_error, handlers, install_configs, uninstall_configs, utils::path::absolute,
};

/// Install a profile
pub fn install(profile_name: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to install profile"),
        arg = profile_name
    );
    let profile = ProfileConfig::load_by_name(&profile_name, global_config).unwrap();

    // Link files
    for link in profile.links.unwrap_or_default() {
        handlers::link::link_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Make directories
    for dir in profile.directories.unwrap_or_default() {
        handlers::files::create_dir(absolute(dir)).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Install packages
    for package in profile.packages.unwrap_or_default() {
        handlers::package::install_package(&package, &global_config.package_add_command)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
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
        arg = profile_name
    );
}

/// Uninstall a profile
pub fn uninstall(profile_name: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to uninstall profile"),
        arg = profile_name
    );
    let profile = ProfileConfig::load_by_name(&profile_name, global_config).unwrap();

    // Unlink files
    for link in profile.links.unwrap_or_default() {
        handlers::link::unlink_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Uninstall packages
    for package in profile.packages.unwrap_or_default() {
        handlers::package::uninstall_package(&package, &global_config.package_remove_command)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run cleanup scripts
    for script in profile.revert_shell.unwrap_or_default() {
        handlers::script::run_script(&script).unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Uninstall configs
    uninstall_configs(profile.configs.unwrap_or_default(), global_config);

    println!(
        "{message}: {arg}",
        message = Green.paint("Finished uninstalling profile"),
        arg = profile_name
    );
}
