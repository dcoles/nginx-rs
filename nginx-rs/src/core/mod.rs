mod buffer;
mod pool;
mod status;
mod string;

pub use buffer::*;
pub use pool::*;
pub use status::*;
pub use string::*;

/// Static string initializer for [`ngx_str_t`].
///
/// The resulting byte string is always nul-terminated (just like a C string).
///
/// [`ngx_str_t`]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
#[macro_export]
macro_rules! ngx_string {
    ($s:expr) => {
        {
            ngx_str_t { len: $s.len(), data: concat!($s, "\0").as_ptr() as *mut u8 }
        }
    };
}

/// Static empty string initializer for [`ngx_str_t`].
///
/// [`ngx_str_t`]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
#[macro_export]
macro_rules! ngx_null_string {
    () => {
        ngx_str_t { len: 0, data: ::std::ptr::null_mut() }
    };
}

/// Static empty configuration directive initializer for [`ngx_command_t`].
///
/// This is typically used to terminate an array of configuration directives.
///
/// [`ngx_command_t`]: https://nginx.org/en/docs/dev/development_guide.html#config_directives
#[macro_export]
macro_rules! ngx_null_command {
    () => {
        ngx_command_t {
            name: $crate::ngx_null_string!(),
            type_: 0,
            set: None,
            conf: 0,
            offset: 0,
            post: ::std::ptr::null_mut(),
        }
    };
}
