/// Builder patterns for configuration objects
use std::path::PathBuf;

use crate::configs::{ConfigConfig, Link, ProfileConfig};

/// Builder for Link configuration
#[derive(Default)]
pub struct LinkBuilder {
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    glob: Option<bool>,
}

impl LinkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from<P: Into<PathBuf>>(mut self, from: P) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to<P: Into<PathBuf>>(mut self, to: P) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn glob(mut self, glob: bool) -> Self {
        self.glob = Some(glob);
        self
    }

    pub fn build(self) -> Link {
        Link {
            from: self.from.expect("from is required"),
            to: self.to.expect("to is required"),
            glob: self.glob,
        }
    }
}

/// Builder for ProfileConfig
#[derive(Default)]
pub struct ProfileConfigBuilder {
    description: Option<String>,
    links: Option<Vec<Link>>,
    directories: Option<Vec<PathBuf>>,
    packages: Option<Vec<String>>,
    shell: Option<Vec<String>>,
    revert_shell: Option<Vec<String>>,
    configs: Option<Vec<String>>,
}

impl ProfileConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = Some(links);
        self
    }

    pub fn add_link(mut self, link: Link) -> Self {
        self.links.get_or_insert_with(Vec::new).push(link);
        self
    }

    pub fn directories(mut self, directories: Vec<PathBuf>) -> Self {
        self.directories = Some(directories);
        self
    }

    pub fn add_directory<P: Into<PathBuf>>(mut self, directory: P) -> Self {
        self.directories
            .get_or_insert_with(Vec::new)
            .push(directory.into());
        self
    }

    pub fn packages(mut self, packages: Vec<String>) -> Self {
        self.packages = Some(packages);
        self
    }

    pub fn add_package<S: Into<String>>(mut self, package: S) -> Self {
        self.packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    pub fn shell(mut self, shell: Vec<String>) -> Self {
        self.shell = Some(shell);
        self
    }

    pub fn add_shell_command<S: Into<String>>(mut self, command: S) -> Self {
        self.shell.get_or_insert_with(Vec::new).push(command.into());
        self
    }

    pub fn revert_shell(mut self, revert_shell: Vec<String>) -> Self {
        self.revert_shell = Some(revert_shell);
        self
    }

    pub fn add_revert_shell_command<S: Into<String>>(mut self, command: S) -> Self {
        self.revert_shell
            .get_or_insert_with(Vec::new)
            .push(command.into());
        self
    }

    pub fn configs(mut self, configs: Vec<String>) -> Self {
        self.configs = Some(configs);
        self
    }

    pub fn add_config<S: Into<String>>(mut self, config: S) -> Self {
        self.configs
            .get_or_insert_with(Vec::new)
            .push(config.into());
        self
    }

    pub fn build(self) -> ProfileConfig {
        ProfileConfig::new(
            self.description,
            self.links,
            self.directories,
            self.packages,
            self.shell,
            self.revert_shell,
            self.configs,
        )
    }
}

/// Builder for ConfigConfig
#[derive(Default)]
pub struct ConfigConfigBuilder {
    description: Option<String>,
    links: Option<Vec<Link>>,
    directories: Option<Vec<PathBuf>>,
    packages: Option<Vec<String>>,
    shell: Option<Vec<String>>,
    revert_shell: Option<Vec<String>>,
}

impl ConfigConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = Some(links);
        self
    }

    pub fn add_link(mut self, link: Link) -> Self {
        self.links.get_or_insert_with(Vec::new).push(link);
        self
    }

    pub fn directories(mut self, directories: Vec<PathBuf>) -> Self {
        self.directories = Some(directories);
        self
    }

    pub fn add_directory<P: Into<PathBuf>>(mut self, directory: P) -> Self {
        self.directories
            .get_or_insert_with(Vec::new)
            .push(directory.into());
        self
    }

    pub fn packages(mut self, packages: Vec<String>) -> Self {
        self.packages = Some(packages);
        self
    }

    pub fn add_package<S: Into<String>>(mut self, package: S) -> Self {
        self.packages
            .get_or_insert_with(Vec::new)
            .push(package.into());
        self
    }

    pub fn shell(mut self, shell: Vec<String>) -> Self {
        self.shell = Some(shell);
        self
    }

    pub fn add_shell_command<S: Into<String>>(mut self, command: S) -> Self {
        self.shell.get_or_insert_with(Vec::new).push(command.into());
        self
    }

    pub fn revert_shell(mut self, revert_shell: Vec<String>) -> Self {
        self.revert_shell = Some(revert_shell);
        self
    }

    pub fn add_revert_shell_command<S: Into<String>>(mut self, command: S) -> Self {
        self.revert_shell
            .get_or_insert_with(Vec::new)
            .push(command.into());
        self
    }

    pub fn build(self) -> ConfigConfig {
        ConfigConfig::new(
            self.description,
            self.links,
            self.directories,
            self.packages,
            self.shell,
            self.revert_shell,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_builder() {
        let link = LinkBuilder::new()
            .from("/test/from")
            .to("/test/to")
            .glob(true)
            .build();

        assert_eq!(link.from, PathBuf::from("/test/from"));
        assert_eq!(link.to, PathBuf::from("/test/to"));
        assert_eq!(link.glob, Some(true));
    }

    #[test]
    fn test_profile_config_builder() {
        let profile = ProfileConfigBuilder::new()
            .description("Test profile")
            .add_package("vim")
            .build();

        assert_eq!(profile.description, Some("Test profile".to_string()));
        assert_eq!(profile.packages, Some(vec!["vim".to_string()]));
    }

    #[test]
    fn test_config_config_builder() {
        let config = ConfigConfigBuilder::new()
            .description("Test config")
            .add_directory("/test/dir")
            .build();

        assert_eq!(config.description, Some("Test config".to_string()));
        assert_eq!(
            config.directories,
            Some(vec![PathBuf::from("/test/dir")])
        );
    }
}
