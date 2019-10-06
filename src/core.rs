use crate::bindings::{
    NGX_OK,
    NGX_ERROR,
    NGX_AGAIN,
    ngx_buf_t,
    ngx_int_t,
    ngx_pool_t,
    ngx_str_t,
    ngx_pcalloc,
};

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
    pub fn set_pos(&mut self, pos: *const u8) {
        unsafe {
            (*self.0).pos = pos as *mut u8;
        }
    }

    pub fn set_last(&mut self, last: *const u8) {
        unsafe {
            (*self.0).last = last as *mut u8;
        }
    }

    pub fn set_static_str(&mut self, str: &str) {
        let pos = str.as_ptr();
        let last = unsafe { pos.offset(str.len() as isize) };

        self.set_pos(pos);
        self.set_last(last);
        self.set_memory(true);
    }

    pub fn set_memory(&mut self, memory: bool) {
        unsafe {
            (*self.0).set_memory(if memory { 1 } else { 0 });
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

pub fn calloc_buf(pool: *mut ngx_pool_t) -> Option<Buffer>
{
    let buf = unsafe { ngx_pcalloc(pool, mem::size_of::<ngx_buf_t>()) } as *mut ngx_buf_t;
    if buf.is_null() {
        return None;
    }
    Some(Buffer(buf))
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
