use std::path::PathBuf;

use structopt::{clap::AppSettings, StructOpt};

// #[derive(Debug, StructOpt)]
// pub struct ProfileConfigSubCommand {
//     #[structopt(short, long)]
//     pub install: Option<Vec<String>>,
//     #[structopt(short, long)]
//     pub uninstall: Option<Vec<String>>,
//     #[structopt(short, long)]
//     pub validate: Option<Vec<String>>,
//     #[structopt(short = "l", long = "list")]
// }

#[derive(Debug, StructOpt)]
pub struct Values {
    pub values: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub enum ProfileConfigSubCommand {
    Install(Values),
    Uninstall(Values),
    List,
}

#[derive(Debug, StructOpt)]
pub enum CompletionsSubCommand {
    Zsh,
    Bash,
}

#[derive(Debug, StructOpt)]
pub enum CliSubcommand {
    Profile(ProfileConfigSubCommand),
    Config(ProfileConfigSubCommand),
    Init {
        #[structopt(short = "r", long = "repo")]
        repo: bool,
        #[structopt(short = "c", long = "config")]
        config: Option<String>,
        #[structopt(short = "p", long = "profile")]
        profile: Option<String>,
    },
    Completions(CompletionsSubCommand),
}

#[derive(StructOpt, Debug)]
pub struct CliOpts {
    /// The level of verbosity
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Sets a custom config file.
    ///
    /// DEFAULT PATH is otherwise determined in this order:
    ///
    ///   - $DOTSY_CONFIG_PATH (environment variable if set)
    ///
    ///   - .dotsyrc.json (in the current directory)
    ///
    ///   - $XDG_CONFIG_HOME/dotsy/dotsyrc.json
    ///
    ///   - $XDG_CONFIG_HOME/dotsy/dotsyrc
    ///
    ///   - $XDG_CONFIG_HOME/dotsyrc.json
    ///
    ///   - $HOME/.dotsyrc.json
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
