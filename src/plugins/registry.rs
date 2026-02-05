/// Plugin registry for managing and discovering plugins
///
/// The registry provides a centralized system for registering, discovering,
/// and executing plugins. It enables modular help information and dynamic
/// plugin discovery.
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{
    error::DotsyError,
    plugins::plugin_trait::Plugin,
    DotsyResult,
};

/// Type alias for the internal plugin storage
type PluginMap = HashMap<String, Box<dyn Plugin>>;
type SharedPluginMap = Arc<RwLock<PluginMap>>;

/// Plugin registry that manages all available plugins
pub struct PluginRegistry {
    plugins: SharedPluginMap,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new plugin in the registry
    ///
    /// # Arguments
    /// * `plugin` - The plugin to register
    ///
    /// # Returns
    /// * `Ok(())` if registration was successful
    /// * `Err` if a plugin with the same name already exists
    pub fn register(&self, plugin: Box<dyn Plugin>) -> DotsyResult<()> {
        let name = plugin.name().to_string();
        let mut plugins = self.plugins.write().map_err(|_| DotsyError::IoError {
            details: "Failed to acquire write lock on plugin registry".to_string(),
        })?;

        if plugins.contains_key(&name) {
            return Err(DotsyError::InvalidConfig {
                details: format!("Plugin '{}' is already registered", name),
            });
        }

        plugins.insert(name, plugin);
        Ok(())
    }

    /// Get a plugin by name
    ///
    /// # Arguments
    /// * `name` - The name of the plugin to retrieve
    ///
    /// # Returns
    /// * `Some` reference to the plugin storage if found
    /// * `None` if no plugin with that name exists
    pub fn get(&self, name: &str) -> Option<SharedPluginMap> {
        let plugins = self.plugins.read().ok()?;
        if plugins.contains_key(name) {
            Some(Arc::clone(&self.plugins))
        } else {
            None
        }
    }

    /// Execute a plugin by name with the given arguments
    ///
    /// # Arguments
    /// * `name` - The name of the plugin to execute
    /// * `args` - Arguments to pass to the plugin
    ///
    /// # Returns
    /// * `Ok(())` if execution was successful
    /// * `Err` if the plugin doesn't exist or execution failed
    pub fn execute(&self, name: &str, args: &[String]) -> DotsyResult<()> {
        let plugins = self.plugins.read().map_err(|_| DotsyError::IoError {
            details: "Failed to acquire read lock on plugin registry".to_string(),
        })?;

        let plugin = plugins.get(name).ok_or_else(|| DotsyError::InvalidConfig {
            details: format!("Plugin '{}' not found in registry", name),
        })?;

        plugin.execute(args)
    }

    /// List all registered plugins
    ///
    /// # Returns
    /// A vector of tuples containing (name, version, description) for each plugin
    pub fn list_plugins(&self) -> DotsyResult<Vec<(String, String, String)>> {
        let plugins = self.plugins.read().map_err(|_| DotsyError::IoError {
            details: "Failed to acquire read lock on plugin registry".to_string(),
        })?;

        Ok(plugins
            .values()
            .map(|p| {
                (
                    p.name().to_string(),
                    p.version().to_string(),
                    p.description().to_string(),
                )
            })
            .collect())
    }

    /// Get help information for a specific plugin
    ///
    /// # Arguments
    /// * `name` - The name of the plugin
    ///
    /// # Returns
    /// A formatted help string with plugin information
    pub fn get_help(&self, name: &str) -> DotsyResult<String> {
        let plugins = self.plugins.read().map_err(|_| DotsyError::IoError {
            details: "Failed to acquire read lock on plugin registry".to_string(),
        })?;

        let plugin = plugins.get(name).ok_or_else(|| DotsyError::InvalidConfig {
            details: format!("Plugin '{}' not found in registry", name),
        })?;

        Ok(format!(
            "{} v{}\n\n{}",
            plugin.name(),
            plugin.version(),
            plugin.description()
        ))
    }

    /// Get help information for all plugins
    ///
    /// # Returns
    /// A formatted help string listing all available plugins
    pub fn get_all_help(&self) -> DotsyResult<String> {
        let plugins_list = self.list_plugins()?;

        if plugins_list.is_empty() {
            return Ok("No plugins available.".to_string());
        }

        let mut help = String::from("Available Plugins:\n\n");
        for (name, version, description) in plugins_list {
            help.push_str(&format!("  {} (v{})\n    {}\n\n", name, version, description));
        }

        Ok(help)
    }

    /// Check if a plugin is registered
    ///
    /// # Arguments
    /// * `name` - The name of the plugin to check
    ///
    /// # Returns
    /// `true` if the plugin is registered, `false` otherwise
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins
            .read()
            .map(|p| p.contains_key(name))
            .unwrap_or(false)
    }

    /// Get the count of registered plugins
    pub fn count(&self) -> usize {
        self.plugins.read().map(|p| p.len()).unwrap_or(0)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::plugin_trait::ExamplePlugin;

    #[test]
    fn test_registry_creation() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(ExamplePlugin::new());

        assert!(registry.register(plugin).is_ok());
        assert_eq!(registry.count(), 1);
        assert!(registry.has_plugin("example"));
    }

    #[test]
    fn test_list_plugins() {
        let registry = PluginRegistry::new();
        registry.register(Box::new(ExamplePlugin::new())).unwrap();

        let plugins = registry.list_plugins().unwrap();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].0, "example");
        assert_eq!(plugins[0].1, "1.0.0");
    }

    #[test]
    fn test_get_help() {
        let registry = PluginRegistry::new();
        registry.register(Box::new(ExamplePlugin::new())).unwrap();

        let help = registry.get_help("example").unwrap();
        assert!(help.contains("example"));
        assert!(help.contains("1.0.0"));
    }

    #[test]
    fn test_execute_plugin() {
        let registry = PluginRegistry::new();
        registry.register(Box::new(ExamplePlugin::new())).unwrap();

        let result = registry.execute("example", &["test".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_nonexistent_plugin() {
        let registry = PluginRegistry::new();
        let result = registry.execute("nonexistent", &[]);
        assert!(result.is_err());
    }
}
