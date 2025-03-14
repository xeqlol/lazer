mod formatter;
mod modules;
mod parser;
mod settings;

use std::collections::HashMap;

use formatter::format_template;

use clap::{Arg, ArgAction, Command};
use modules::get_modules;
use parser::parse_template;
use settings::Settings;

const ZSH_INIT: &str = include_str!("./init.zsh");

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
    let settings =
        Settings::new().unwrap_or_else(|settings| panic!("wrong settings format: {}", settings));

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("{}", ZSH_INIT);
        }
        Some(("prompt", _)) => {
            let prompt_template = parse_template(&settings.format);
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
        _ => unreachable!(),
    }
}
