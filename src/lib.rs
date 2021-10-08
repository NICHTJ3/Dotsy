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

fn install_config(config: String, _global_config: &DotsyConfig) {
    let config = configs::ConfigConfig::load_by_name(&config).unwrap();
    println!("{:?}", config)
    // TODO: How do I access the links? They're an optional Vec<Links>?
}

// TODO: Find a way to cache the load of the rcfile for the life of the program
pub fn load_rcfile() -> DotsyConfig {
    let rcfile_path = defaults::fallback_path().unwrap();

    let config = configs::DotsyConfig::load(rcfile_path).unwrap();

    return config;
}
