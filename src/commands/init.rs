use crate::configs::{self, ConfigFile};

pub fn init(repo: bool, config_name: Option<String>, profile_name: Option<String>) {
    if let Some(config_name) = &config_name {
        configs::ConfigConfig::create(config_name).unwrap();
    }
    if let Some(profile_name) = &profile_name {
        configs::ProfileConfig::create(profile_name).unwrap();
    }
    if (profile_name.is_none() && config_name.is_none()) || repo {
        configs::DotsyConfig::create("./.dotsyrc.json").unwrap();
    }
}
