extern crate glob;

use dotsy::{
    cli::{self, Cli},
    configs::{self, ConfigFile},
    DotsyResult,
};
use glob::glob;
use std::{path::PathBuf, process};
use structopt::StructOpt;

fn main() {
    let opt = Cli::from_args();
    match handle_subcommands(opt) {
        Ok(_) => {
            process::exit(0);
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn handle_subcommands(opt: Cli) -> DotsyResult<()> {
    if let Some(subcmd) = opt.cmd {
        match subcmd {
            cli::CliSubcommand::Init {
                repo: _,
                config: config_name,
                profile: profile_name,
            } => {
                if config_name.is_some() {
                    // TODO: Cleaner way to create config from config name
                    let config_name = config_name.unwrap().as_str().to_owned();
                    let config_filename = configs::ConfigConfig::create_file_name(&config_name);
                    configs::ConfigConfig::create(PathBuf::from(config_filename)).unwrap();
                } else if profile_name.is_some() {
                    let profile_name = profile_name.unwrap().as_str().to_owned();
                    let profile_filename = configs::ProfileConfig::create_file_name(&profile_name);
                    configs::ProfileConfig::create(PathBuf::from(profile_filename)).unwrap();
                } else {
                    configs::DotsyConfig::create(PathBuf::from("./.dotsyrc.json")).unwrap();
                }
            }
            cli::CliSubcommand::Profile(opts) => {
                let config = dotsy::load_rcfile();

                if opts.install.is_some() {
                    dotsy::install_profiles(opts.install.unwrap(), &config);
                }
                if opts.uninstall.is_some() {
                    println!("Ye");
                }
                if opts.validate.is_some() {
                    println!("Ye");
                }
            }
            cli::CliSubcommand::Config(opts) => {
                let config = dotsy::load_rcfile();

                if opts.install.is_some() {
                    dotsy::install_configs(opts.install.unwrap(), &config);
                }
                if opts.uninstall.is_some() {
                    println!("Ye");
                }
                if opts.validate.is_some() {
                    println!("Ye");
                }
            }
            cli::CliSubcommand::List { configs, profiles } => {
                // TODO: Only show names not full file path
                // TODO: Break out into functions
                let config = dotsy::load_rcfile();
                let configs_found = glob(
                    &config
                        .dotfiles
                        .join(&config.configs_dir)
                        .join("./*.config.json")
                        .into_os_string()
                        .to_str()
                        .unwrap(),
                )
                .expect("Failed to read glob pattern");

                let profiles_found = glob(
                    &config
                        .dotfiles
                        .join(&config.profiles_dir)
                        .join("./*.profile.json")
                        .into_os_string()
                        .to_str()
                        .unwrap(),
                )
                .expect("Failed to read glob pattern");

                let mut configs_peekable = configs_found.into_iter().peekable();
                let mut profiles_peekable = profiles_found.into_iter().peekable();

                let have_configs = configs_peekable.peek().is_some();
                let have_profiles = profiles_peekable.peek().is_some();

                if configs || !profiles && have_configs {
                    println!("Available Configs to install");
                    configs_peekable.for_each(|e| {
                        println!(" - {}", e.unwrap().display());
                    });
                    println!();
                } else if have_profiles {
                    println!("Available Profiles to install");
                    profiles_peekable.for_each(|e| {
                        println!(" - {}", e.unwrap().display());
                    });
                    println!();
                } else {
                    // TODO: Should this be done with error handling
                    println!("No configs or profiles found to install!!");
                }
            }
        }
    }
    Ok(())
}
