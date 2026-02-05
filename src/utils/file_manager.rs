/// FileManager: Centralized file and directory operations
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::{error::DotsyError, DotsyResult};

/// FileManager handles all file system operations
pub struct FileManager;

impl FileManager {
    /// Create a new FileManager instance
    pub fn new() -> Self {
        Self
    }

    /// Create a directory, including all parent directories
    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> DotsyResult<()> {
        let path = path.as_ref();
        if path.exists() {
            if path.is_dir() {
                // Directory already exists, this is OK
                return Ok(());
            } else {
                return Err(DotsyError::FileAlreadyExists {
                    file: path.to_path_buf(),
                });
            }
        }

        fs::create_dir_all(path).map_err(|e| DotsyError::IoError {
            details: format!("Failed to create directory {}: {}", path.display(), e),
        })?;

        Ok(())
    }

    /// Create a symlink from `from` to `to`
    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to: Q,
    ) -> DotsyResult<()> {
        let from = from.as_ref();
        let to = to.as_ref();

        // Check if source exists
        if !from.exists() {
            return Err(DotsyError::CouldntCreateSymLink {
                from: from.to_path_buf(),
                to: to.to_path_buf(),
                reason: "Source file or directory does not exist".to_string(),
            });
        }

        // Check if target already exists
        if to.exists() || to.is_symlink() {
            return Err(DotsyError::FileAlreadyExists {
                file: to.to_path_buf(),
            });
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = to.parent() {
            self.create_directory(parent)?;
        }

        // Create the symlink
        unix_fs::symlink(from, to).map_err(|e| DotsyError::CouldntCreateSymLink {
            from: from.to_path_buf(),
            to: to.to_path_buf(),
            reason: format!("Failed to create symlink: {}", e),
        })?;

        Ok(())
    }

    /// Remove a symlink
    pub fn remove_symlink<P: AsRef<Path>>(&self, path: P) -> DotsyResult<()> {
        let path = path.as_ref();

        // Check if it's a symlink
        if !path.is_symlink() {
            return Err(DotsyError::Unlink {
                link: path.to_path_buf(),
                reason: "Not a symlink".to_string(),
            });
        }

        fs::remove_file(path).map_err(|e| DotsyError::Unlink {
            link: path.to_path_buf(),
            reason: format!("Failed to remove symlink: {}", e),
        })?;

        Ok(())
    }

    /// Check if path exists
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    /// Check if path is a symlink
    pub fn is_symlink<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_symlink()
    }

    /// Check if path is a directory
    pub fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    /// Check if path is a file
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_manager_creation() {
        let fm = FileManager::new();
        assert!(!fm.exists("/nonexistent/path"));
    }

    #[test]
    fn test_file_manager_default() {
        let fm = FileManager::default();
        assert!(!fm.is_symlink("/nonexistent/path"));
    }
}
