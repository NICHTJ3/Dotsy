pub mod cli;
pub mod configs;
pub mod defaults;
pub mod error;
pub mod macros;

use configs::{ConfigFile, DotsyConfig};
use error::DotsyError;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

pub fn install_configs(configs: Vec<String>, global_config: &DotsyConfig) {
    for config in configs {
        install_config(config, global_config)
    }
}

fn install_config(config: String, global_config: &DotsyConfig) {
    println!("Installing: {}, with config: {:?}", config, global_config);
}

// TODO: Find a way to cache the load of the rcfile for the life of the program
pub fn load_rcfile() -> DotsyConfig {
    // TODO: Find a cleaner way to do this (it's a lot of unwraps etc
    let rcfile_path = defaults::fallback_path()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let config = configs::DotsyConfig::load(&rcfile_path).unwrap();

    return config;
}
