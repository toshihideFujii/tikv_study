
#[macro_export]
macro_rules! thd_name {
  ($name:expr) => {{
    $crate::get_tag_from_thread_name().map(|tag| format!("{}::{}", $name, tag))
      .unwrap_or_else(|| $name.to_owned())
  }};
}