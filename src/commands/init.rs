use crate::configs::{self, ConfigFile};

pub fn init(repo: bool, config_name: Option<String>, profile_name: Option<String>) {
    if config_name.is_some() {
        let config_name = config_name.as_ref().unwrap().to_string();
        configs::ConfigConfig::create(&config_name).unwrap();
    }
    if profile_name.is_some() {
        let profile_name = profile_name.as_ref().unwrap().to_string();
        configs::ProfileConfig::create(&profile_name).unwrap();
    }
    if (profile_name.is_none() && config_name.is_none()) || repo {
        configs::DotsyConfig::create("./.dotsyrc.json").unwrap();
    }
}
