/// Plugin trait for extensibility
/// 
/// The Plugin trait defines the interface for extending Dotsy with custom functionality.
/// Plugins can be used to add new handlers, processors, or integrations.

use crate::DotsyResult;

/// Core plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Returns the name of the plugin
    fn name(&self) -> &str;

    /// Returns the version of the plugin
    fn version(&self) -> &str;

    /// Returns a description of what the plugin does
    fn description(&self) -> &str;

    /// Initialize the plugin
    /// 
    /// This is called when the plugin is first loaded and can be used
    /// to set up any necessary state or resources.
    fn initialize(&mut self) -> DotsyResult<()> {
        Ok(())
    }

    /// Execute the plugin's main functionality
    /// 
    /// # Arguments
    /// * `args` - Arguments passed to the plugin
    fn execute(&self, args: &[String]) -> DotsyResult<()>;

    /// Cleanup the plugin
    /// 
    /// This is called when the plugin is being unloaded and should
    /// release any held resources.
    fn cleanup(&mut self) -> DotsyResult<()> {
        Ok(())
    }
}

/// Handler plugin trait for custom file/package handlers
pub trait HandlerPlugin: Plugin {
    /// Handle installation
    fn install(&self, target: &str) -> DotsyResult<()>;

    /// Handle uninstallation
    fn uninstall(&self, target: &str) -> DotsyResult<()>;
}

/// Example plugin demonstrating the plugin interface
pub struct ExamplePlugin {
    name: String,
    initialized: bool,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            name: "example".to_string(),
            initialized: false,
        }
    }
}

impl Default for ExamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "An example plugin demonstrating the plugin interface"
    }

    fn initialize(&mut self) -> DotsyResult<()> {
        self.initialized = true;
        println!("Example plugin initialized");
        Ok(())
    }

    fn execute(&self, args: &[String]) -> DotsyResult<()> {
        println!("Example plugin executing with args: {:?}", args);
        Ok(())
    }

    fn cleanup(&mut self) -> DotsyResult<()> {
        self.initialized = false;
        println!("Example plugin cleaned up");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_plugin() {
        let mut plugin = ExamplePlugin::new();
        assert_eq!(plugin.name(), "example");
        assert_eq!(plugin.version(), "1.0.0");
        assert!(!plugin.initialized);

        assert!(plugin.initialize().is_ok());
        assert!(plugin.initialized);

        assert!(plugin.execute(&["test".to_string()]).is_ok());
        assert!(plugin.cleanup().is_ok());
        assert!(!plugin.initialized);
    }
}
