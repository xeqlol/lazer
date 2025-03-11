use std::collections::HashMap;

use crate::parser::{Expression, Style, Template};

fn format_string(strig: String, style: Style) -> String {
    let bold_code = if style.bold { "1;" } else { "" };
    let fg_code = style.fg.map_or(String::new(), |c| format!("38;2;{};", c));
    let bg_code = style.bg.map_or(String::new(), |c| format!("48;2;{};", c));

    format!(
        "%{{\x1b[{}{}{}m{}\x1b[0m%}}",
        bold_code, fg_code, bg_code, strig
    )
}

// "%{{{}%}}",
// "\x1b[38;2;255;82;197;48;2;155;106;0mTRUECOLOR\x1b[0m"

// fn format_string(string: String, style: Style) -> String {
//     let bold_part = if style.bold { "%B" } else { "" };
//     let fg_part = style
//         .fg
//         .as_ref()
//         .map_or("".to_string(), |c| format!("%F{{{}}}", c));
//     let bg_part = style
//         .bg
//         .as_ref()
//         .map_or("".to_string(), |c| format!("%K{{{}}}", c));

//     format!("%{{{}{}{}{}%f%k%b%}}", bold_part, fg_part, bg_part, string)
// }

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
                    let formatted = format_string(format, text_group.style);

                    result.push_str(formatted.as_str());
                }
            }
        }

        result
    }

    format(template.expressions, variables)
}
