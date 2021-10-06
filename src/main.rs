mod cli;

use std::process;

use cli::Cli;
use dotsy::DotsyResult;
use structopt::StructOpt;

fn main() {
    let opt = Cli::from_args();
    match handle_subcommands(opt) {
        Ok(_) => {
            println!("Done");
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
                config: _,
                profile: _,
            } => {
                println!("Trying to initialize: Repo , Config , Profile ");
            }
            _ => {
                panic!("Oh no this isn't implemented yet");
            }
        }
    }
    Ok(())
}
