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
        .join("*.profile.json");

    let profiles = glob(profiles_regex.to_str().unwrap()).expect("Failed to read glob pattern");

    println!("Available Profiles to install");
    profiles.filter_map(Result::ok).for_each(|profile| {
        // Strip both extensions e.g. json, profile from the filename before printing it
        if let Some(file_name) = profile.with_extension("").with_extension("").file_name() {
            println!(" - {}", file_name.to_str().unwrap());
        }
    });
    println!();
}
