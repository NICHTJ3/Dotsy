/// ProfileManager: Handles profile operations using the plugin system
use ansi_term::Colour::Green;

use crate::{
    configs::{DotsyConfig, ProfileConfig},
    dotsy_log_error,
    handlers::plugin_handlers::{DirectoryHandlerPlugin, PackageHandlerPlugin, ScriptHandlerPlugin},
    install_configs,
    plugins::plugin_trait::{HandlerPlugin, Plugin},
    uninstall_configs,
};

/// Install a profile using plugin handlers
pub fn install(profile_name: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to install profile"),
        arg = profile_name
    );
    let profile = ProfileConfig::load_by_name(&profile_name, global_config).unwrap();

    // Link files using handlers (these are more complex and handled separately)
    for link in profile.links.unwrap_or_default() {
        crate::handlers::link::link_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Make directories using directory handler plugin
    let dir_handler = DirectoryHandlerPlugin::new();
    for dir in profile.directories.unwrap_or_default() {
        let dir_path = crate::utils::path::absolute(dir);
        dir_handler
            .install(dir_path.to_str().unwrap())
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Install packages using package handler plugin
    let package_handler = PackageHandlerPlugin::new(
        global_config.package_add_command.clone(),
        global_config.package_remove_command.clone(),
    );
    for package in profile.packages.unwrap_or_default() {
        package_handler
            .install(&package)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run scripts using script handler plugin
    let script_handler = ScriptHandlerPlugin::new();
    for script in profile.shell.unwrap_or_default() {
        script_handler
            .execute(&[script])
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Install configs
    install_configs(profile.configs.unwrap_or_default(), global_config);

    println!(
        "{message}: {arg}",
        message = Green.paint("Finished installing profile"),
        arg = profile_name
    );
}

/// Uninstall a profile using plugin handlers
pub fn uninstall(profile_name: String, global_config: &DotsyConfig) {
    println!(
        "{message}: {arg}",
        message = Green.paint("Attempting to uninstall profile"),
        arg = profile_name
    );
    let profile = ProfileConfig::load_by_name(&profile_name, global_config).unwrap();

    // Unlink files using handlers
    for link in profile.links.unwrap_or_default() {
        crate::handlers::link::unlink_file(link, global_config)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Uninstall packages using package handler plugin
    let package_handler = PackageHandlerPlugin::new(
        global_config.package_add_command.clone(),
        global_config.package_remove_command.clone(),
    );
    for package in profile.packages.unwrap_or_default() {
        package_handler
            .uninstall(&package)
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Run cleanup scripts using script handler plugin
    let script_handler = ScriptHandlerPlugin::new();
    for script in profile.revert_shell.unwrap_or_default() {
        script_handler
            .execute(&[script])
            .unwrap_or_else(|e| dotsy_log_error!("{}", e));
    }

    // Uninstall configs
    uninstall_configs(profile.configs.unwrap_or_default(), global_config);

    println!(
        "{message}: {arg}",
        message = Green.paint("Finished uninstalling profile"),
        arg = profile_name
    );
}
