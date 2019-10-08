use crate::bindings::*;

use std::{slice, ptr};
use std::str;
use std::mem;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Status(pub ngx_int_t);

pub const OK: Status = Status(NGX_OK as ngx_int_t);
pub const ERROR: Status = Status(NGX_ERROR as ngx_int_t);
pub const AGAIN: Status = Status(NGX_AGAIN as ngx_int_t);

impl Status {
    pub fn is_ok(&self) -> bool {
        self == &OK
    }
}

pub struct Buffer(pub *mut ngx_buf_t);

impl Buffer {
    pub fn create_temp(pool: *mut ngx_pool_t, size: usize) -> Option<Buffer> {
        assert!(!pool.is_null());
        let buf = unsafe { ngx_create_temp_buf(pool, size) };
        if buf.is_null() {
            return None;
        }

        Some(Buffer(buf))
    }

    pub fn create_from_static_str(pool: *mut ngx_pool_t, str: &'static str) -> Option<Buffer> {
        assert!(!pool.is_null());
        let buf = unsafe { ngx_calloc_buf(pool) };
        if buf.is_null() {
            return None;
        }

        let mut buf = Buffer(buf);
        // We cast away cost, but buffers with the memory flag are read-only
        let start = str.as_ptr() as *mut u8;
        let end = unsafe { start.offset(str.len() as isize) };

        unsafe {
            (*buf.0).start = start;
            (*buf.0).pos = start;
            (*buf.0).last = end;
            (*buf.0).end = end;
        }
        buf.set_memory(true);

        Some(buf)
    }

    pub fn create_from_str(pool: *mut ngx_pool_t, str: &str) -> Option<Buffer>
    {
        let mut buf = Buffer::create_temp(pool, str.len())?;
        unsafe {
            ptr::copy_nonoverlapping(str.as_ptr(), (*buf.0).pos, str.len());
            (*buf.0).last = (*buf.0).pos.offset(str.len() as isize);
        }
        Some(buf)
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.0).pos, self.len()) }
    }

    pub fn as_bytes_mut(&self) -> &mut [u8] {
        assert!(!self.is_memory());
        unsafe { slice::from_raw_parts_mut((*self.0).pos, self.len()) }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let pos = (*self.0).pos;
            let last = (*self.0).last;
            assert!(last > pos);
            usize::wrapping_sub(last as _, pos as _)
        }
    }

    pub fn is_memory(&self) -> bool {
        unsafe {
            (*self.0).memory() != 0
        }
    }

    pub fn is_temporary(&self) -> bool {
        unsafe {
            (*self.0).temporary() != 0
        }
    }

    fn set_memory(&mut self, memory: bool) {
        unsafe {
            (*self.0).set_memory(if memory { 1 } else { 0 });
        }
    }

    fn set_temporary(&mut self, temporary: bool) {
        unsafe {
            (*self.0).set_temporary(if temporary { 1 } else { 0 });
        }
    }

    pub fn set_last_buf(&mut self, last: bool) {
        unsafe {
            (*self.0).set_last_buf(if last { 1 } else { 0 });
        }
    }

    pub fn set_last_in_chain(&mut self, last: bool) {
        unsafe {
            (*self.0).set_last_in_chain(if last { 1 } else { 0 });
        }
    }
}

pub unsafe fn ngx_alloc_buf(pool: *mut ngx_pool_t) -> *mut ngx_buf_t {
    ngx_palloc(pool, mem::size_of::<ngx_buf_t>()) as *mut ngx_buf_t
}

pub unsafe fn ngx_calloc_buf(pool: *mut ngx_pool_t) -> *mut ngx_buf_t {
    ngx_pcalloc(pool, mem::size_of::<ngx_buf_t>()) as *mut ngx_buf_t
}

impl ngx_str_t {
    pub fn to_str(&self) -> &str {
        let bytes = unsafe { slice::from_raw_parts(self.data, self.len) };
        str::from_utf8(bytes).unwrap_or_default()
    }

    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}
