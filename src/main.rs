extern crate ansi_term;
extern crate glob;

use dotsy::{
    cli::{self, Cli},
    commands, dotsy_log_error, DotsyResult,
};
use std::process;
use structopt::StructOpt;

fn main() {
    let opt = Cli::from_args();
    match handle_subcommands(opt) {
        Ok(_) => {
            process::exit(0);
        }
        Err(err) => {
            dotsy_log_error!("{}", err);
            process::exit(1);
        }
    }
}

fn handle_subcommands(opt: Cli) -> DotsyResult<()> {
    let config = dotsy::load_rcfile().unwrap();
    if let Some(subcmd) = opt.cmd {
        match subcmd {
            cli::CliSubcommand::Init {
                repo,
                config,
                profile,
            } => commands::init::init(repo, config, profile),
            cli::CliSubcommand::Profile(opts) => match opts {
                cli::ProfileConfigSubCommand::Uninstall(opts) => {
                    commands::profile::uninstall(opts.values, &config);
                }
                cli::ProfileConfigSubCommand::Install(opts) => {
                    commands::profile::install(opts.values, &config);
                }
                cli::ProfileConfigSubCommand::List => {
                    commands::profile::list(&config);
                }
            },
            cli::CliSubcommand::Config(opts) => match opts {
                cli::ProfileConfigSubCommand::Uninstall(opts) => {
                    commands::config::uninstall(opts.values, &config);
                }
                cli::ProfileConfigSubCommand::Install(opts) => {
                    commands::config::install(opts.values, &config);
                }
                cli::ProfileConfigSubCommand::List => {
                    commands::profile::list(&config);
                }
            },
        }
    }
    Ok(())
}
