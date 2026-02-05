pub mod plugin_commands;

use std::path::PathBuf;

use structopt::{
    clap::{AppSettings, Shell},
    StructOpt,
};

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
pub enum PluginSubCommand {
    /// List all available plugins
    List,
    /// Show help for a plugin
    Help {
        /// Plugin name (optional, shows all if not provided)
        name: Option<String>,
    },
    /// Execute a plugin
    Execute {
        /// Plugin name
        name: String,
        /// Arguments to pass to the plugin
        #[structopt(raw = true)]
        args: Vec<String>,
    },
}

impl From<CompletionsSubCommand> for Shell {
    fn from(cmd: CompletionsSubCommand) -> Self {
        match cmd {
            CompletionsSubCommand::Zsh => Self::Zsh,
            CompletionsSubCommand::Bash => Self::Bash,
        }
    }
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
    /// Plugin management commands
    Plugin {
        #[structopt(subcommand)]
        cmd: PluginSubCommand,
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
