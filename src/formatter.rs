use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "spec.pest"]
pub struct TemplateParser;

fn format_string(string: String, styles: Vec<&str>) -> String {
    let mut fmt = string;

    for style in styles {
        if style.starts_with("f:") {
            fmt = format!(r"%F{{{}}}{}%f", style.trim()[2..].to_string(), fmt)
        }

        if style.starts_with("b:") {
            fmt = format!(r"%K{{{}}}{}%k", style.trim()[2..].to_string(), fmt)
        }

        if style.trim() == "b" {
            fmt = format!(r"%B{}%b", fmt)
        }
    }

    fmt
}

pub fn format_template(template: &str, variables: &HashMap<&str, String>) -> String {
    let pairs = TemplateParser::parse(Rule::file, template).unwrap();

    fn parse(pairs: pest::iterators::Pairs<Rule>, variables: &HashMap<&str, String>) -> String {
        let mut result = String::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::text => result.push_str(pair.as_str().replace(r"\", "").as_str()),
                Rule::variable => {
                    let variable = &pair.as_str()[1..]; // remove $ sign at start

                    if let Some(value) = variables.get(variable) {
                        result.push_str(value)
                    } else {
                        result.push_str(pair.as_str())
                    }
                }
                Rule::text_group => {
                    let mut pair = pair.into_inner();
                    let format = parse(pair.next().unwrap().into_inner(), variables);
                    let style = pair
                        .next()
                        .unwrap()
                        .as_str()
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>();
                    let formatted = format_string(format, style);

                    result.push_str(formatted.as_str());
                }
                _ => {}
            }
        }

        result
    }

    parse(pairs, variables)
}
