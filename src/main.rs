mod formatter;
mod modules;
mod parser;

use std::collections::HashMap;

use formatter::format_template;

use clap::{Arg, ArgAction, Command};
use modules::get_modules;
use parser::parse_template;

const ZSH_INIT: &str = include_str!("./init.zsh");

const PROMPT_TEMPLATE: &str = "[](f:yellow)$user[](f:yellow b:green)$dir[](f:green b:blue)$git[](f:blue)\n[:](f:yellow)[:](f:green)[>](f:blue) ";

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
                let prompt_template = parse_template(PROMPT_TEMPLATE);
                let modules = get_modules();
                let mut rendered_modules: HashMap<String, String> = HashMap::new();

                for variable in prompt_template.variables.clone() {
                    let module = modules.get(variable.as_str());

                    if module.is_none() {
                        continue;
                    }

                    let module = (module.unwrap())();

                    if module.is_err() {
                        continue;
                    }

                    let inner_module = module.unwrap();

                    if inner_module.is_none() {
                        continue;
                    }

                    let rendered_module = inner_module.unwrap();

                    let template = parse_template(&rendered_module.template);
                    let formatted_module = format_template(template, &rendered_module.variables);

                    rendered_modules.insert(variable.to_string(), formatted_module);
                }

                let formatted_prompt = format_template(prompt_template, &rendered_modules);

                print!("{}", formatted_prompt);
            }
        }
        _ => unreachable!(),
    }
}
