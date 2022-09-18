extern crate glob;
use crate::{configs::DotsyConfig, install_profile, uninstall_profile};
use glob::glob;

pub fn install(profiles: Vec<String>, config: &DotsyConfig) {
    for profile in profiles {
        install_profile(profile, config)
    }
}

pub fn uninstall(profiles: Vec<String>, config: &DotsyConfig) {
    for profile in profiles {
        uninstall_profile(profile, config);
    }
}

pub fn list(config: &DotsyConfig) {
    let profiles_regex = &config
        .dotfiles
        .join(&config.profiles_dir)
        .join("*.profile.json")
        .into_os_string()
        .to_owned();
    let profiles_found =
        glob(&profiles_regex.to_str().unwrap()).expect("Failed to read glob pattern");
    let profiles = profiles_found.into_iter().peekable();
    println!("Available Profiles to install");
    profiles.for_each(|e| {
        println!(" - {}", e.unwrap().display());
    });
    println!();
}
