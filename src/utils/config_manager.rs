/// ConfigManager: Centralized configuration file management
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{error::DotsyError, DotsyResult};

/// ConfigManager handles loading and saving configuration files
pub struct ConfigManager;

impl ConfigManager {
    /// Create a new ConfigManager instance
    pub fn new() -> Self {
        Self
    }

    /// Load a configuration from a file
    pub fn load<T, P>(&self, path: P) -> DotsyResult<T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let file = File::open(path).map_err(|_| DotsyError::ConfigNotAvailable {
            config: path.to_path_buf(),
        })?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|e| DotsyError::JsonError {
            details: format!("Failed to parse JSON from {}: {}", path.display(), e),
        })
    }

    /// Save a configuration to a file
    pub fn save<T, P>(&self, config: &T, path: P) -> DotsyResult<()>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let serialized = serde_json::to_string_pretty(config).map_err(|e| {
            DotsyError::JsonError {
                details: format!("Failed to serialize config: {}", e),
            }
        })?;

        let mut file = File::create(path).map_err(|e| DotsyError::IoError {
            details: format!("Failed to create file {}: {}", path.display(), e),
        })?;

        file.write_all(serialized.as_bytes())
            .map_err(|e| DotsyError::IoError {
                details: format!("Failed to write to file {}: {}", path.display(), e),
            })?;

        Ok(())
    }

    /// Validate that a path exists and is a file
    pub fn validate_path<P: AsRef<Path>>(&self, path: P) -> DotsyResult<PathBuf> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(DotsyError::ConfigNotAvailable {
                config: path.to_path_buf(),
            });
        }
        if !path.is_file() {
            return Err(DotsyError::InvalidConfig {
                details: format!("{} is not a file", path.display()),
            });
        }
        Ok(path.to_path_buf())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager_creation() {
        let cm = ConfigManager::new();
        assert!(cm.validate_path("/nonexistent/path").is_err());
    }

    #[test]
    fn test_config_manager_default() {
        let _cm = ConfigManager::default();
    }
}
