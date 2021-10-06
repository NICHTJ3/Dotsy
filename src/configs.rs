use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DotsyConfig {
    dotfiles: PathBuf,
    package_add_command: String,
    package_remove_command: String,
}

impl DotsyConfig {
    pub fn new(
        dotfiles: PathBuf,
        package_add_command: Option<String>,
        package_remove_command: Option<String>,
    ) -> Self {
        Self {
            dotfiles,
            package_add_command: package_add_command.unwrap_or(String::from("brew add {}")),
            package_remove_command: package_remove_command
                .unwrap_or(String::from("brew remove {}")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    from: PathBuf,
    to: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    description: String,
    links: Vec<Link>,
    directories: Vec<PathBuf>,
    packages: Vec<String>,
    shell: Vec<String>,
    revert_shell: Vec<String>,
    configs: Vec<String>,
}

impl ProfileConfig {
    pub fn new(
        description: String,
        links: Vec<Link>,
        directories: Vec<PathBuf>,
        packages: Vec<String>,
        shell: Vec<String>,
        revert_shell: Vec<String>,
        configs: Vec<String>,
    ) -> Self {
        Self {
            description,
            links,
            directories,
            packages,
            shell,
            revert_shell,
            configs,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigConfig {
    description: String,
    links: Vec<Link>,
    directories: Vec<PathBuf>,
    packages: Vec<String>,
    shell: Vec<String>,
    revert_shell: Vec<String>,
}

impl ConfigConfig {
    pub fn new(
        description: String,
        links: Vec<Link>,
        directories: Vec<PathBuf>,
        packages: Vec<String>,
        shell: Vec<String>,
        revert_shell: Vec<String>,
    ) -> Self {
        Self {
            description,
            links,
            directories,
            packages,
            shell,
            revert_shell,
        }
    }
}
