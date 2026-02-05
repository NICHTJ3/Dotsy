/// CLI commands for plugin management
///
/// This module provides CLI integration for the plugin system,
/// including plugin discovery, execution, and help generation.
use crate::{
    configs::DotsyConfig,
    handlers::plugin_handlers::{
        DirectoryHandlerPlugin, PackageHandlerPlugin, ScriptHandlerPlugin,
    },
    plugins::registry::PluginRegistry,
    DotsyResult,
};

/// Initialize the plugin registry with all built-in plugins
///
/// This function registers all built-in handler plugins with the registry
/// so they can be discovered and executed through the CLI.
///
/// # Arguments
/// * `config` - The global Dotsy configuration
///
/// # Returns
/// A populated PluginRegistry with all built-in plugins
pub fn initialize_registry(config: &DotsyConfig) -> DotsyResult<PluginRegistry> {
    let registry = PluginRegistry::new();

    // Register package handler plugin
    let package_plugin = PackageHandlerPlugin::new(
        config.package_add_command.clone(),
        config.package_remove_command.clone(),
    );
    registry.register(Box::new(package_plugin))?;

    // Register script handler plugin
    let script_plugin = ScriptHandlerPlugin::new();
    registry.register(Box::new(script_plugin))?;

    // Register directory handler plugin
    let dir_plugin = DirectoryHandlerPlugin::new();
    registry.register(Box::new(dir_plugin))?;

    Ok(registry)
}

/// Execute a plugin command from the CLI
///
/// # Arguments
/// * `registry` - The plugin registry
/// * `plugin_name` - The name of the plugin to execute
/// * `args` - Arguments to pass to the plugin
pub fn execute_plugin(
    registry: &PluginRegistry,
    plugin_name: &str,
    args: &[String],
) -> DotsyResult<()> {
    registry.execute(plugin_name, args)
}

/// Display help for a specific plugin or all plugins
///
/// # Arguments
/// * `registry` - The plugin registry
/// * `plugin_name` - Optional plugin name. If None, shows help for all plugins
pub fn show_plugin_help(registry: &PluginRegistry, plugin_name: Option<&str>) -> DotsyResult<()> {
    let help_text = match plugin_name {
        Some(name) => registry.get_help(name)?,
        None => registry.get_all_help()?,
    };

    println!("{}", help_text);
    Ok(())
}

/// List all registered plugins
///
/// # Arguments
/// * `registry` - The plugin registry
pub fn list_plugins(registry: &PluginRegistry) -> DotsyResult<()> {
    let plugins = registry.list_plugins()?;

    if plugins.is_empty() {
        println!("No plugins registered.");
        return Ok(());
    }

    println!("Registered Plugins:\n");
    for (name, version, description) in plugins {
        println!("  {} (v{})", name, version);
        println!("    {}\n", description);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_config() -> DotsyConfig {
        DotsyConfig {
            dotfiles: PathBuf::from("/tmp/dotfiles"),
            profiles_dir: PathBuf::from("profiles"),
            configs_dir: PathBuf::from("configs"),
            package_add_command: "echo install {}".to_string(),
            package_remove_command: "echo remove {}".to_string(),
        }
    }

    #[test]
    fn test_initialize_registry() {
        let config = get_test_config();
        let registry = initialize_registry(&config).unwrap();

        assert!(registry.has_plugin("package-handler"));
        assert!(registry.has_plugin("script-handler"));
        assert!(registry.has_plugin("directory-handler"));
        assert_eq!(registry.count(), 3);
    }

    #[test]
    fn test_list_plugins() {
        let config = get_test_config();
        let registry = initialize_registry(&config).unwrap();

        let result = list_plugins(&registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_plugin_help() {
        let config = get_test_config();
        let registry = initialize_registry(&config).unwrap();

        let result = show_plugin_help(&registry, Some("package-handler"));
        assert!(result.is_ok());

        let result_all = show_plugin_help(&registry, None);
        assert!(result_all.is_ok());
    }
}
