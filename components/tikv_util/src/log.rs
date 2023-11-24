

#[macro_export]
macro_rules! error {
  (?$e:expr; $l:literal) => {
    ::slog_global::error!($l "err" => ?$e, "err_code" => %error_code::ErrorCodeExt::error_code(&$e))
  };  
  (%$e:expr; $l:literal) => {
    ::slog_global::error!($l "err" => ?$e, "err_code" => %error_code::ErrorCodeExt::error_code(&$e))
  };
  (?$e:expr; $($args:tt)+) => {
    ::slog_global::error!($($args)+ "err" => ?$e, "err_code" => %error_code::ErrorCodeExt::error_code(&$e))
  };
  (%$e:expr; $($args:tt)+) => {
    ::slog_global::error!($($args)+ "err" => %$e, "err_code" => %error_code::ErrorCodeExt::error_code(&$e))
  };
  ($($args:tt)+) => {
    ::slog_global::error!($($args)+)
  };
}

#[macro_export]
macro_rules! warn(($($args:tt)+) => {
  ::slog_global::warn!($($args)+)
};);

#[macro_export]
macro_rules! info(($($args:tt)+) => {
  ::slog_global::info!($($args)+)
};);

#[macro_export]
macro_rules! debug(($($args:tt)+) => {
  ::slog_global::debug!($($args)+)
};);

#[macro_export]
macro_rules! trace(($($args:tt)+) => {
  ::slog_global::trace!($($args)+)
};);