use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::DotsyResult;

// TODO: Do this stuff better

pub trait ConfigFile {
    fn load(path: PathBuf) -> DotsyResult<Self>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let v: Self = serde_json::from_reader(reader).unwrap();
        Ok(v)
    }
    fn create(path: PathBuf) -> DotsyResult<Self>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DotsyConfig {
    dotfiles: PathBuf,
    profiles_dir: PathBuf,
    configs_dir: PathBuf,
    package_add_command: String,
    package_remove_command: String,
}

impl ConfigFile for DotsyConfig {
    fn create(path: PathBuf) -> DotsyResult<Self> {
        let config = DotsyConfig {
            dotfiles: PathBuf::from("~/Dotfiles"),
            package_add_command: "brew add {}".to_string(),
            package_remove_command: "brew remove {}".to_string(),
            profiles_dir: PathBuf::from("profiles"),
            configs_dir: PathBuf::from("configs"),
        };

        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let mut file = File::create(path).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    pub description: Option<String>,
    pub links: Option<Vec<Link>>,
    pub directories: Option<Vec<PathBuf>>,
    pub packages: Option<Vec<String>>,
    pub shell: Option<Vec<String>>,
    pub revert_shell: Option<Vec<String>>,
    pub configs: Option<Vec<String>>,
    parent_dir: PathBuf,
}

impl ProfileConfig {
    pub fn new(
        description: Option<String>,
        links: Option<Vec<Link>>,
        directories: Option<Vec<PathBuf>>,
        packages: Option<Vec<String>>,
        shell: Option<Vec<String>>,
        revert_shell: Option<Vec<String>>,
        configs: Option<Vec<String>>,
        parent_dir: PathBuf,
    ) -> Self {
        Self {
            description,
            links,
            directories,
            packages,
            shell,
            revert_shell,
            configs,
            parent_dir,
        }
    }

    pub fn create_file_name(name: &str) -> String {
        format!("./{}.profile.json", name)
    }

    pub fn load_by_name(name: &str) -> DotsyResult<Self> {
        let file_name = PathBuf::from(Self::create_file_name(name));

        Self::load(file_name)
    }
}

impl ConfigFile for ProfileConfig {
    fn create(path: PathBuf) -> DotsyResult<Self> {
        let config = ProfileConfig::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            PathBuf::from("~/Dotfiles/configs"),
        );

        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let mut file = File::create(path).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigConfig {
    pub description: Option<String>,
    pub links: Option<Vec<Link>>,
    pub directories: Option<Vec<PathBuf>>,
    pub packages: Option<Vec<String>>,
    pub shell: Option<Vec<String>>,
    pub revert_shell: Option<Vec<String>>,
}

impl ConfigConfig {
    pub fn new(
        description: Option<String>,
        links: Option<Vec<Link>>,
        directories: Option<Vec<PathBuf>>,
        packages: Option<Vec<String>>,
        shell: Option<Vec<String>>,
        revert_shell: Option<Vec<String>>,
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
    pub fn create_file_name(name: &str) -> String {
        format!("./{}.config.json", name)
    }

    pub fn load_by_name(name: &str) -> DotsyResult<Self> {
        let file_name = PathBuf::from(Self::create_file_name(name));

        Self::load(file_name)
    }
}

impl ConfigFile for ConfigConfig {
    fn create(path: PathBuf) -> DotsyResult<Self> {
        let config = ConfigConfig::new(None, None, None, None, None, None);
        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let mut file = File::create(path).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        Ok(config)
    }
}
