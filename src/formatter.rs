use std::collections::HashMap;

use crate::parser::{Expression, Template};

// TODO: add #xxxxxx colors support (translate them to 255/16 colors automatically)
fn format_string(string: String, styles: Vec<&str>) -> String {
    let mut fmt = string;

    // FIX: foreground not working, need to fix that
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

pub fn format_template(template: Template, variables: &HashMap<String, String>) -> String {
    fn format(expressions: Vec<Expression>, variables: &HashMap<String, String>) -> String {
        let mut result = String::new();

        for expression in expressions {
            match expression {
                Expression::Text(text) => result.push_str(text.replace(r"\", "").as_str()),
                Expression::Variable(var_name) => {
                    if let Some(value) = variables.get(var_name.as_str()) {
                        result.push_str(value)
                    } else {
                        result.push_str("")
                    }
                }
                Expression::TextGroup(text_group) => {
                    let format = format(text_group.format, variables);
                    let style = text_group
                        .style
                        .as_str()
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>();
                    let formatted = format_string(format, style);

                    result.push_str(formatted.as_str());
                }
            }
        }

        result
    }

    format(template.expressions, variables)
}
