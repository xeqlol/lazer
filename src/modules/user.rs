use std::collections::HashMap;

use crate::segment::Segment;

pub fn get_user_segment<'a>() -> Segment<'a> {
    let username = whoami::username();
    let mut variables = HashMap::new();
    variables.insert("user", username);

    Segment {
        template: r"\[[$user](f:red b)\]".to_string(),
        variables,
    }
}
