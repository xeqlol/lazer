pub mod dir;
pub mod git;
pub mod user;

use dir::get_dir_module;
use git::get_git_module;
use user::get_user_module;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Module {
    pub template: String,
    pub variables: HashMap<String, String>,
}

type ModuleRender = dyn Fn() -> Result<Option<Module>, anyhow::Error>;

// TODO: get rid of crazy types, rewrite to struct Modules and

pub fn get_modules<'a>() -> HashMap<&'a str, Box<ModuleRender>> {
    let mut modules: HashMap<&'a str, Box<ModuleRender>> = HashMap::new();

    modules.insert("user", Box::new(get_user_module));
    modules.insert("dir", Box::new(get_dir_module));
    modules.insert("git", Box::new(get_git_module));

    modules
}
