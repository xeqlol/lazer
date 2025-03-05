use std::{collections::HashMap, env};

use crate::segment::Segment;

pub fn get_dir_segment<'a>() -> Segment<'a> {
    let curren_dir = env::current_dir()
        .expect("the wtf with current dir?")
        .into_os_string()
        .into_string()
        .expect("wtf problem due to string convert");

    let home_dir = dirs::home_dir()
        .expect("some stuff with home dir")
        .into_os_string()
        .into_string()
        .unwrap();
    let dir = curren_dir.replace(&home_dir, "~");

    let mut variables = HashMap::new();
    variables.insert("directory", dir.clone());

    Segment {
        template: r"\[at [$directory](f:yellow b)\]".to_string(),
        variables,
    }
}
