use std::path::PathBuf;

use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
pub struct ProfileConfigSubCommand {
    #[structopt(short, long)]
    install: Option<Vec<String>>,
    #[structopt(short, long)]
    uninstall: Option<Vec<String>>,
}

#[derive(Debug, StructOpt)]
pub enum CliSubcommand {
    Profile(ProfileConfigSubCommand),
    Config(ProfileConfigSubCommand),
    #[structopt(alias = "ls")]
    List {
        #[structopt(short = "c", long = "configs")]
        configs: Option<bool>,
        #[structopt(short = "p", long = "profiles")]
        profiles: Option<bool>,
    },
    Init {
        #[structopt(short = "r", long = "repo")]
        repo: Option<bool>,
        #[structopt(short = "c", long = "configs")]
        config: Option<bool>,
        #[structopt(short = "p", long = "profiles")]
        profile: Option<bool>,
    },
}

#[derive(StructOpt, Debug)]
pub struct CliOpts {
    /// The level of verbosity
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// TODO: Set this up
    /// Sets a custom config file.
    ///
    /// DEFAULT PATH is otherwise determined in this order:
    ///
    ///   - $DOTSY_CONFIG_PATH (environment variable if set)
    ///
    ///   - dotsy.toml (in the current directory)
    ///
    ///   - $XDG_CONFIG_HOME/dotsy/config.toml
    ///
    ///   - $XDG_CONFIG_HOME/dotsy/config
    ///
    ///   - $XDG_CONFIG_HOME/dotsy.toml
    ///
    ///   - $HOME/.dotsy.toml
    ///
    ///   - $HOME/.dotsy
    ///
    #[structopt(short = "c", long = "config-file", env = "DOTSY_CONFIG_PATH")]
    pub path: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::SubcommandsNegateReqs, setting = AppSettings::TrailingVarArg, author)]
/// A simple script management CLI
pub struct Cli {
    #[structopt(flatten)]
    pub opts: CliOpts,

    /// dotsy subcommands
    #[structopt(subcommand)]
    pub cmd: Option<CliSubcommand>,
}
