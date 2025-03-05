use crate::formatter::format_template;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Segment<'a> {
    pub template: String,
    pub variables: HashMap<&'a str, String>,
}

impl<'a> Segment<'a> {
    pub fn render(&self) -> String {
        format_template(&self.template, &self.variables)
    }
}
