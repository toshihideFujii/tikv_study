use std::thread;


#[macro_use]
pub mod log;
#[macro_use]
pub mod macros;
pub mod sys;
pub mod thread_group;
pub mod time;

pub fn get_tag_from_thread_name() -> Option<String> {
  thread::current()
    .name()
    .and_then(|name| name.split("::").skip(1).last())
    .map(From::from)
}