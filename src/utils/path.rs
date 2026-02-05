/// Path utilities for handling file paths and operations
use std::fs;
use std::path::PathBuf;

/// Convert a path to an absolute path, expanding tildes
pub fn absolute(base: PathBuf) -> PathBuf {
    let path_str = base.to_string_lossy();
    match shellexpand::tilde(&path_str) {
        std::borrow::Cow::Borrowed(s) => PathBuf::from(s),
        std::borrow::Cow::Owned(s) => PathBuf::from(s),
    }
}

/// Check if a link (symlink) exists at the given path
pub fn link_exists(path: &PathBuf) -> bool {
    fs::symlink_metadata(path).is_ok()
}

/// Check if the path is a symlink
pub fn is_symlink(path: &PathBuf) -> bool {
    match fs::symlink_metadata(path) {
        Ok(metadata) => metadata.file_type().is_symlink(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symlink_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        assert!(!is_symlink(&path));
    }

    #[test]
    fn test_link_exists_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        assert!(!link_exists(&path));
    }
}
