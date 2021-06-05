use crate::bindings::*;

use std::marker::PhantomData;
use std::slice;
use std::str::{self, Utf8Error};
use std::borrow::Cow;

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

/// Representation of a borrowed [Nginx string].
///
/// This ensures that the lifetime of strings are correctly tracked.
///
/// [Nginx string]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
pub struct NgxStr<'a>(ngx_str_t, PhantomData<&'a [u8]>);

impl<'a> NgxStr<'a> {
    /// Create an [`NgxStr`] from an [`ngx_str_t`].
    ///
    /// The string must point to a valid block of memory of at least `len` bytes
    /// that must remain valid and constant for the lifetime of the returned [`NgxStr`].
    ///
    /// [`ngx_str_t`]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
    pub unsafe fn from_ngx_str(str: ngx_str_t) -> Self {
        NgxStr(str, PhantomData)
    }

    /// Access the [`NgxStr`] as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.data, self.0.len) }
    }

    /// Yields a `&str` slice if the [`NgxStr`] contains valid UTF-8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts an [`NgxStr`] into a [`Cow<str>`], replacing invalid UTF-8 sequences.
    ///
    /// See [`String::from_utf8_lossy`].
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    /// Returns `true` if the [`NgxStr`] is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.0.len == 0
    }
}

impl From<&str> for NgxStr<'_> {
    fn from(s: &str) -> Self {
        NgxStr(ngx_str_t { len: s.len(), data: s.as_ptr() as *mut u_char }, PhantomData)
    }
}

impl AsRef<[u8]> for NgxStr<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Default for NgxStr<'_> {
    fn default() -> Self {
        NgxStr(ngx_null_string!(), PhantomData)
    }
}
