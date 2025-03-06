use itertools::Itertools;
mod formatter;
mod modules;
mod segment;

use clap::{Arg, ArgAction, Command};
use modules::get_modules;

const ZSH_INIT: &str = include_str!("./init.zsh");

const SEPARATOR: &str = "";

fn cli() -> Command {
    Command::new("lazer")
        .about("The L A Z E R Prompt")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Provide complete prompt config for shell"))
        .subcommand(
            Command::new("prompt")
                .about("Provide string for prompt")
                .arg(
                    Arg::new("right")
                        .long("right")
                        .action(ArgAction::Set)
                        .num_args(0..1),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("{}", ZSH_INIT);
        }
        Some(("prompt", sync_matches)) => {
            if sync_matches.contains_id("right") {
                // todo
                unreachable!()
            } else {
                let segments = get_modules();

                let fmt = segments
                    .iter()
                    .cloned()
                    .filter(|segment| segment.is_some())
                    .map(|segment| segment.unwrap().render())
                    .join(SEPARATOR);

                print!("{}\n→ ", fmt);
            }
        }
        _ => unreachable!(),
    }
}
