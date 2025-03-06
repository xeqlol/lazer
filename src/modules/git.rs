use git2::{ErrorCode, Repository};
use std::{collections::HashMap, env};

use crate::segment::Segment;

const DEFAULT_TEMPLATE: &str = r"\[git [$branch$is_dirty](f:blue b)\]";

pub fn get_git_segment<'a>() -> Option<Segment<'a>> {
    let repo = Repository::open(env::current_dir().unwrap());

    if repo.is_err() {
        return None;
    }

    let repo = repo.unwrap();

    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        _ => None,
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    let branch: &str = head.unwrap_or("HEAD");
    let is_dirty = !repo.statuses(None).unwrap().is_empty();

    let mut variables = HashMap::new();
    variables.insert("branch", branch.to_string());
    variables.insert(
        "is_dirty",
        if is_dirty {
            "*".to_string()
        } else {
            "".to_string()
        },
    );

    Some(Segment {
        template: DEFAULT_TEMPLATE.to_string(),
        variables,
    })
}
