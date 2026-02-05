extern crate ansi_term;
extern crate glob;

use dotsy::cli::plugin_commands;
use dotsy::cli::{CliSubcommand, PluginSubCommand, ProfileConfigSubCommand::*};
use dotsy::{
    cli::{Cli, CliSubcommand::*},
    commands, dotsy_log_error, DotsyResult,
};
use std::io::stdout;
use std::process;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    match handle_subcommand(cli.cmd) {
        Ok(_) => {
            process::exit(0);
        }
        Err(err) => {
            dotsy_log_error!("{}", err);
            process::exit(1);
        }
    }
}

fn handle_subcommand(cmd: Option<CliSubcommand>) -> DotsyResult<()> {
    if let Some(subcmd) = cmd {
        match subcmd {
            Init {
                repo,
                config,
                profile,
            } => commands::init::init(repo, config, profile),
            Profile(opts) => {
                let config = dotsy::load_rcfile()?;
                match opts {
                    Uninstall(opts) => {
                        commands::profile::uninstall(opts.values, &config);
                    }
                    Install(opts) => {
                        commands::profile::install(opts.values, &config);
                    }
                    List => {
                        commands::profile::list(&config);
                    }
                }
            }
            Config(opts) => {
                let config = dotsy::load_rcfile()?;
                match opts {
                    Uninstall(opts) => {
                        commands::config::uninstall(opts.values, &config);
                    }
                    Install(opts) => {
                        commands::config::install(opts.values, &config);
                    }
                    List => {
                        commands::config::list(&config);
                    }
                }
            }
            Plugin { cmd } => {
                // For plugin commands that don't require full config, provide defaults
                let config = dotsy::load_rcfile().unwrap_or_else(|_| {
                    use std::path::PathBuf;
                    dotsy::configs::DotsyConfig {
                        dotfiles: PathBuf::from("~/.dotfiles"),
                        profiles_dir: PathBuf::from("profiles"),
                        configs_dir: PathBuf::from("configs"),
                        package_add_command: "echo install {}".to_string(),
                        package_remove_command: "echo remove {}".to_string(),
                    }
                });
                let registry = plugin_commands::initialize_registry(&config)?;

                match cmd {
                    PluginSubCommand::List => plugin_commands::list_plugins(&registry)?,
                    PluginSubCommand::Help { name } => {
                        plugin_commands::show_plugin_help(&registry, name.as_deref())?
                    }
                    PluginSubCommand::Execute { name, args } => {
                        plugin_commands::execute_plugin(&registry, &name, &args)?
                    }
                }
            }
            Completions(opt) => {
                Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), opt.into(), &mut stdout())
            }
        }
    }
    Ok(())
}
