use std::collections::HashMap;

use crate::modules::Module;

const DEFAULT_TEMPLATE: &str = r"[$user ](b:#ffcb00 f:black)";

pub fn get_user_module() -> Result<Option<Module>, anyhow::Error> {
    let username = whoami::username();
    let mut variables = HashMap::new();
    variables.insert("user".to_string(), username);

    Ok(Some(Module {
        template: DEFAULT_TEMPLATE.to_string(),
        variables,
    }))
}
