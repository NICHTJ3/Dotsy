extern crate ansi_term;
extern crate glob;

use dotsy::cli::{CliSubcommand, ProfileConfigSubCommand::*};
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
    let config = dotsy::load_rcfile()?;

    if let Some(subcmd) = cmd {
        match subcmd {
            Init {
                repo,
                config,
                profile,
            } => commands::init::init(repo, config, profile),
            Profile(opts) => match opts {
                Uninstall(opts) => {
                    commands::profile::uninstall(opts.values, &config);
                }
                Install(opts) => {
                    commands::profile::install(opts.values, &config);
                }
                List => {
                    commands::profile::list(&config);
                }
            },
            Config(opts) => match opts {
                Uninstall(opts) => {
                    commands::config::uninstall(opts.values, &config);
                }
                Install(opts) => {
                    commands::config::install(opts.values, &config);
                }
                List => {
                    commands::config::list(&config);
                }
            },
            Completions(opt) => {
                Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), opt.into(), &mut stdout())
            }
        }
    }
    Ok(())
}
