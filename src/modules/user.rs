use std::collections::HashMap;

use crate::segment::Segment;

const DEFAULT_TEMPLATE: &str = r"\[[$user](f:red b)\]";

pub fn get_user_segment<'a>() -> Option<Segment<'a>> {
    let username = whoami::username();
    let mut variables = HashMap::new();
    variables.insert("user", username);

    Some(Segment {
        template: DEFAULT_TEMPLATE.to_string(),
        variables,
    })
}
