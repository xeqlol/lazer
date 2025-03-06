mod dir;
mod git;
mod user;

use dir::get_dir_segment;
use git::get_git_segment;
use user::get_user_segment;

use crate::segment::Segment;

pub fn get_modules<'a>() -> Vec<Option<Segment<'a>>> {
    vec![get_user_segment(), get_dir_segment(), get_git_segment()]
}
