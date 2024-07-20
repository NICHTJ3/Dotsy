use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

// TODO: Do this stuff better
// - A lot of the config/profile functionallity is shared (should it be a trait?)
// - The paths for creation etc should be taken from the dotsyrc

pub trait ConfigFile {
    fn load(path: PathBuf) -> DotsyResult<Self>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let file = {
            let this = File::open(&path);
            match this {
                Ok(t) => t,
                Err(..) => return dotsy_err!(DotsyError::ConfigNotAvailable { config: (path) }),
            }
        };
        let reader = BufReader::new(file);

        let v: Self = serde_json::from_reader(reader).unwrap();
        Ok(v)
    }
    fn create(file_name: &str) -> DotsyResult<Self>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DotsyConfig {
    pub dotfiles: PathBuf,
    pub profiles_dir: PathBuf,
    pub configs_dir: PathBuf,
    pub package_add_command: String,
    pub package_remove_command: String,
}

impl ConfigFile for DotsyConfig {
    fn create(file_name: &str) -> DotsyResult<Self> {
        let config = DotsyConfig {
            dotfiles: PathBuf::from("~/Dotfiles"),
            package_add_command: "brew add {}".to_string(),
            package_remove_command: "brew remove {}".to_string(),
            profiles_dir: PathBuf::from("profiles"),
            configs_dir: PathBuf::from("configs"),
        };

        let serialized = serde_json::to_string_pretty(&config).unwrap();

        let mut file = File::create(PathBuf::from(file_name)).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub from: PathBuf,
    pub to: PathBuf,
    pub glob: Option<bool>,
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

    pub fn create_file_name(name: &str) -> String {
        format!("./{}.profile.json", name)
    }

    pub fn load_by_name(name: &str, global_config: &DotsyConfig) -> DotsyResult<Self> {
        let profiles_dir = PathBuf::from(&global_config.dotfiles).join(&global_config.profiles_dir);
        let file_name = profiles_dir.join(Self::create_file_name(name));

        Self::load(file_name)
    }
}

impl ConfigFile for ProfileConfig {
    fn create(profile_name: &str) -> DotsyResult<Self> {
        let path = PathBuf::from(ProfileConfig::create_file_name(profile_name));
        let config = ProfileConfig::new(None, None, None, None, None, None, None);

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
    fn create_file_name(name: &str) -> String {
        format!("./{}.config.json", name)
    }

    pub fn load_by_name(name: &str, global_config: &DotsyConfig) -> DotsyResult<Self> {
        let configs_dir = PathBuf::from(&global_config.dotfiles).join(&global_config.configs_dir);
        let file_name = configs_dir.join(Self::create_file_name(name));

        Self::load(file_name)
    }
}

impl ConfigFile for ConfigConfig {
    fn create(config_name: &str) -> DotsyResult<Self> {
        let path = PathBuf::from(ConfigConfig::create_file_name(config_name));
        let config = ConfigConfig::new(None, None, None, None, None, None);
        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let mut file = File::create(path).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        Ok(config)
    }
}
