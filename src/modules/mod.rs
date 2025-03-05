use dir::get_dir_segment;
use user::get_user_segment;

use crate::segment::Segment;

mod dir;
mod user;

pub fn get_modules<'a>() -> Vec<Segment<'a>> {
    vec![get_user_segment(), get_dir_segment()]
}
