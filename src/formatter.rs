use std::collections::HashMap;

use crate::parser::{Expression, Style, Template};

const FOREGROUND_PREFIX: &str = "3";
const BACKGROUND_PREFIX: &str = "4";

fn color_name_to_256() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("black", "0");
    map.insert("red", "1");
    map.insert("green", "2");
    map.insert("yellow", "3");
    map.insert("blue", "4");
    map.insert("magenta", "5");
    map.insert("cyan", "6");
    map.insert("white", "7");
    map.insert("bright_black", "0");
    map.insert("bright_red", "1");
    map.insert("bright_green", "2");
    map.insert("bright_yellow", "3");
    map.insert("bright_blue", "4");
    map.insert("bright_magenta", "5");
    map.insert("bright_cyan", "6");
    map.insert("bright_white", "7");
    map
}

fn hex_to_rgb_string(hex: &str) -> Option<String> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None;
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

    Some(format!("{};{};{}", r, g, b))
}

// prefix is 3 for foreground, 4 for background
fn parse_color(color: &str, prefix: &str) -> String {
    let color_map = color_name_to_256();

    if let Some(&code) = color_map.get(&color) {
        if color.starts_with("bright_") {
            let prefix = if prefix == FOREGROUND_PREFIX {
                "9"
            } else {
                "10"
            };
            return format!("{}{}", prefix, code);
        }

        return format!("{}{}", prefix, code);
    }

    match color.parse::<u8>() {
        Ok(code) => return format!("{}8;5;{}", prefix, code),
        Err(_) => {
            if let Some(hex) = hex_to_rgb_string(color) {
                return format!("{}8;2;{}", prefix, hex);
            } else {
                panic!("invalid color code: {}", color)
            }
        }
    }
}

fn wrap_string_for_zsh(string: String) -> String {
    format!("%{{{}%}}", string)
}

fn format_string(string: String, style: Style) -> String {
    let mut format_parts = Vec::new();

    if style.bold {
        format_parts.push("1".to_string());
    }

    if let Some(fg) = style.fg {
        format_parts.push(parse_color(&fg, FOREGROUND_PREFIX));
    }

    if let Some(bg) = style.bg {
        format_parts.push(parse_color(&bg, BACKGROUND_PREFIX))
    }

    let format = format_parts.join(";");
    let color = wrap_string_for_zsh(format!("\x1b[{format}m"));
    let reset = wrap_string_for_zsh("\x1b[0m".to_string());

    format!("{color}{string}{reset}")
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
                    let formatted = format_string(format, text_group.style);

                    result.push_str(formatted.as_str());
                }
            }
        }

        result
    }

    format(template.expressions, variables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color() {
        assert_eq!(parse_color("black", FOREGROUND_PREFIX), "30");
        assert_eq!(parse_color("blue", BACKGROUND_PREFIX), "44");
        assert_eq!(parse_color("bright_yellow", FOREGROUND_PREFIX), "93");
        assert_eq!(parse_color("bright_red", BACKGROUND_PREFIX), "101");
    }

    #[test]
    fn test_numeric_color() {
        assert_eq!(parse_color("32", FOREGROUND_PREFIX), "38;5;32".to_string());
        assert_eq!(
            parse_color("100", BACKGROUND_PREFIX),
            "48;5;100".to_string()
        );
    }

    #[test]
    fn test_hex_color() {
        assert_eq!(
            parse_color("#ff0000", FOREGROUND_PREFIX),
            "38;2;255;0;0".to_string()
        );
        assert_eq!(
            parse_color("#00ff00", BACKGROUND_PREFIX),
            "48;2;0;255;0".to_string()
        );
        assert_eq!(
            parse_color("#0000ff", FOREGROUND_PREFIX),
            "38;2;0;0;255".to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_color() {
        parse_color("invalid", FOREGROUND_PREFIX);
    }
}
