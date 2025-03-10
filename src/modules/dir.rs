use std::{collections::HashMap, env};

use crate::modules::Module;

const DEFAULT_TEMPLATE: &str = r"[ $directory ](b:#94c237 f:white)";

pub fn get_dir_module() -> Result<Option<Module>, anyhow::Error> {
    let current_dir = env::current_dir()?.into_os_string().into_string();

    if current_dir.is_err() {
        return Ok(None);
    }

    let current_dir = current_dir.unwrap();

    let home_dir = dirs::home_dir()
        .expect("some stuff with home dir")
        .into_os_string()
        .into_string();

    if home_dir.is_err() {
        return Ok(None);
    }

    let home_dir = home_dir.unwrap();

    let dir = current_dir.replace(&home_dir, "~");

    let mut variables = HashMap::new();
    variables.insert("directory".to_string(), dir.clone());

    Ok(Some(Module {
        template: DEFAULT_TEMPLATE.to_string(),
        variables,
    }))
}
