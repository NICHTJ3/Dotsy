use dotsy::{
    cli::{self, Cli},
    configs::{self, ConfigFile},
    DotsyResult,
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
                config,
                profile,
            } => {
                if config.is_some() {
                    configs::ConfigConfig::create(config.unwrap().as_str()).unwrap();
                } else if profile.is_some() {
                    configs::ProfileConfig::create(profile.unwrap().as_str()).unwrap();
                } else {
                    configs::DotsyConfig::create("./.dotsyrc.json").unwrap();
                }
            }
            cli::CliSubcommand::Profile(opts) => {
                let _config = dotsy::load_rcfile();

                if opts.install.is_some() {
                    println!("Ye");
                }
                if opts.uninstall.is_some() {
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
            }
            cli::CliSubcommand::List {
                configs: _,
                profiles: _,
            } => todo!(),
        }
    }
    Ok(())
}
