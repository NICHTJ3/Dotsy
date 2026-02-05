/// Plugin implementations for existing handlers
/// 
/// This module provides plugin-based implementations of the core handler functionality,
/// allowing handlers to be used through the unified plugin interface.
use crate::{
    configs::{DotsyConfig, Link},
    error::DotsyError,
    handlers::{files, link, package, script},
    plugins::plugin_trait::{HandlerPlugin, Plugin},
    DotsyResult,
};
use std::path::PathBuf;

/// Package handler plugin for installing/uninstalling packages
pub struct PackageHandlerPlugin {
    install_command: String,
    uninstall_command: String,
}

impl PackageHandlerPlugin {
    pub fn new(install_command: String, uninstall_command: String) -> Self {
        Self {
            install_command,
            uninstall_command,
        }
    }
}

impl Plugin for PackageHandlerPlugin {
    fn name(&self) -> &str {
        "package-handler"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Handles package installation and uninstallation"
    }

    fn execute(&self, args: &[String]) -> DotsyResult<()> {
        if args.is_empty() {
            return Err(DotsyError::InvalidConfig {
                details: "Package name required".to_string(),
            });
        }
        self.install(&args[0])
    }
}

impl HandlerPlugin for PackageHandlerPlugin {
    fn install(&self, target: &str) -> DotsyResult<()> {
        package::install_package(target, &self.install_command)
    }

    fn uninstall(&self, target: &str) -> DotsyResult<()> {
        package::uninstall_package(target, &self.uninstall_command)
    }
}

/// Script handler plugin for running shell scripts
pub struct ScriptHandlerPlugin;

impl ScriptHandlerPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScriptHandlerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ScriptHandlerPlugin {
    fn name(&self) -> &str {
        "script-handler"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Executes shell scripts"
    }

    fn execute(&self, args: &[String]) -> DotsyResult<()> {
        if args.is_empty() {
            return Err(DotsyError::InvalidConfig {
                details: "Script command required".to_string(),
            });
        }
        script::run_script(&args[0])
    }
}

/// Link handler plugin for creating/removing symlinks
pub struct LinkHandlerPlugin {
    global_config: DotsyConfig,
}

impl LinkHandlerPlugin {
    pub fn new(global_config: DotsyConfig) -> Self {
        Self { global_config }
    }
}

impl Plugin for LinkHandlerPlugin {
    fn name(&self) -> &str {
        "link-handler"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Creates and removes symlinks"
    }

    fn execute(&self, args: &[String]) -> DotsyResult<()> {
        if args.len() < 2 {
            return Err(DotsyError::InvalidConfig {
                details: "Link requires 'from' and 'to' paths".to_string(),
            });
        }
        let link_data = Link {
            from: PathBuf::from(&args[0]),
            to: PathBuf::from(&args[1]),
            glob: args.get(2).map(|s| s == "true"),
        };
        link::link_file(link_data, &self.global_config)
    }
}

impl HandlerPlugin for LinkHandlerPlugin {
    fn install(&self, _target: &str) -> DotsyResult<()> {
        // For link handler, target would be a serialized Link structure
        // This is a simplified version
        Err(DotsyError::InvalidConfig {
            details: "Use execute with from/to paths instead".to_string(),
        })
    }

    fn uninstall(&self, _target: &str) -> DotsyResult<()> {
        // For link handler, target would be a serialized Link structure
        Err(DotsyError::InvalidConfig {
            details: "Use dedicated unlink functionality".to_string(),
        })
    }
}

/// Directory handler plugin for creating directories
pub struct DirectoryHandlerPlugin;

impl DirectoryHandlerPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DirectoryHandlerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DirectoryHandlerPlugin {
    fn name(&self) -> &str {
        "directory-handler"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Creates directories"
    }

    fn execute(&self, args: &[String]) -> DotsyResult<()> {
        if args.is_empty() {
            return Err(DotsyError::InvalidConfig {
                details: "Directory path required".to_string(),
            });
        }
        files::create_dir(PathBuf::from(&args[0]))
    }
}

impl HandlerPlugin for DirectoryHandlerPlugin {
    fn install(&self, target: &str) -> DotsyResult<()> {
        files::create_dir(PathBuf::from(target))
    }

    fn uninstall(&self, _target: &str) -> DotsyResult<()> {
        // Directories typically aren't removed during uninstall
        // to preserve user data
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_handler_plugin() {
        let plugin = PackageHandlerPlugin::new(
            "echo install {}".to_string(),
            "echo uninstall {}".to_string(),
        );
        assert_eq!(plugin.name(), "package-handler");
        assert_eq!(plugin.version(), "1.0.0");
    }

    #[test]
    fn test_script_handler_plugin() {
        let plugin = ScriptHandlerPlugin::new();
        assert_eq!(plugin.name(), "script-handler");
        assert_eq!(plugin.version(), "1.0.0");
    }

    #[test]
    fn test_directory_handler_plugin() {
        let plugin = DirectoryHandlerPlugin::new();
        assert_eq!(plugin.name(), "directory-handler");
        assert_eq!(plugin.version(), "1.0.0");
    }
}
