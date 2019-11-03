mod buffer;
mod pool;
mod status;
mod string;

pub use buffer::*;
pub use pool::*;
pub use status::*;
pub use string::*;

pub const fn size_of_val<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

#[macro_export]
macro_rules! ngx_string {
    ($x:expr) => {
        ngx_str_t { len: $crate::core::size_of_val($x) - 1, data: $x.as_ptr() as *mut u8 }
    };
}

#[macro_export]
macro_rules! ngx_null_string {
    () => {
        ngx_str_t { len: 0, data: ::std::ptr::null_mut() }
    };
}

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
