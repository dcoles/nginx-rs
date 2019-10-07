use crate::bindings::*;

use std::slice;
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
        let start = str.as_ptr();
        let end = unsafe { start.offset(str.len() as isize) };

        buf.set_start(start);
        buf.set_pos(start);
        buf.set_last(end);
        buf.set_end(end);
        buf.set_memory(true);

        Some(buf)
    }

    fn set_start(&mut self, start: *const u8) {
        unsafe {
            (*self.0).start = start as *mut u8;
        }
    }

    fn set_pos(&mut self, pos: *const u8) {
        unsafe {
            (*self.0).pos = pos as *mut u8;
        }
    }

    fn set_last(&mut self, last: *const u8) {
        unsafe {
            (*self.0).last = last as *mut u8;
        }
    }

    fn set_end(&mut self, end: *const u8) {
        unsafe {
            (*self.0).end = end as *mut u8;
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
