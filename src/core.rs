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

pub trait Buffer {
    fn as_ngx_buf(&self) -> *const ngx_buf_t;

    fn as_ngx_buf_mut(&self) -> *mut ngx_buf_t;

    fn as_bytes(&self) -> &[u8];

    fn len(&self) -> usize;

    fn set_last_buf(&mut self, last: bool);

    fn set_last_in_chain(&mut self, last: bool);
}

pub trait MutableBuffer: Buffer {
    fn as_bytes_mut(&self) -> &mut [u8];
}

pub struct TemporaryBuffer(*mut ngx_buf_t);

impl TemporaryBuffer {
    pub fn create_temp(pool: *mut ngx_pool_t, size: usize) -> Option<TemporaryBuffer> {
        assert!(!pool.is_null());
        let buf = unsafe { ngx_create_temp_buf(pool, size) };
        if buf.is_null() {
            return None;
        }

        Some(TemporaryBuffer(buf))
    }

    pub fn create_from_str(pool: *mut ngx_pool_t, str: &str) -> Option<TemporaryBuffer>
    {
        let mut buf = TemporaryBuffer::create_temp(pool, str.len())?;
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
}

impl Buffer for TemporaryBuffer {
    fn as_ngx_buf(&self) -> *const ngx_buf_t {
        self.0
    }

    fn as_ngx_buf_mut(&self) -> *mut ngx_buf_t {
        self.0
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.0).pos, self.len()) }
    }

    fn len(&self) -> usize {
        unsafe {
            let pos = (*self.0).pos;
            let last = (*self.0).last;
            assert!(last > pos);
            usize::wrapping_sub(last as _, pos as _)
        }
    }

    fn set_last_buf(&mut self, last: bool) {
        unsafe {
            (*self.0).set_last_buf(if last { 1 } else { 0 });
        }
    }

    fn set_last_in_chain(&mut self, last: bool) {
        unsafe {
            (*self.0).set_last_in_chain(if last { 1 } else { 0 });
        }
    }
}

impl MutableBuffer for TemporaryBuffer {
    fn as_bytes_mut(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut((*self.0).pos, self.len()) }
    }
}

pub struct MemoryBuffer(*mut ngx_buf_t);

impl MemoryBuffer {
    pub fn create_from_static_str(pool: *mut ngx_pool_t, str: &'static str) -> Option<TemporaryBuffer> {
        assert!(!pool.is_null());
        let buf = unsafe { ngx_calloc_buf(pool) };
        if buf.is_null() {
            return None;
        }

        let mut buf = TemporaryBuffer(buf);
        // We cast away cost, but buffers with the memory flag are read-only
        let start = str.as_ptr() as *mut u8;
        let end = unsafe { start.offset(str.len() as isize) };

        unsafe {
            (*buf.0).start = start;
            (*buf.0).pos = start;
            (*buf.0).last = end;
            (*buf.0).end = end;
            (*buf.0).set_memory(1);
        }

        Some(buf)
    }
}

impl Buffer for MemoryBuffer {
    fn as_ngx_buf(&self) -> *const ngx_buf_t {
        return self.0
    }

    fn as_ngx_buf_mut(&self) -> *mut ngx_buf_t {
        return self.0
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.0).pos, self.len()) }
    }

    fn len(&self) -> usize {
        unsafe {
            let pos = (*self.0).pos;
            let last = (*self.0).last;
            assert!(last > pos);
            usize::wrapping_sub(last as _, pos as _)
        }
    }

    fn set_last_buf(&mut self, last: bool) {
        unsafe {
            (*self.0).set_last_buf(if last { 1 } else { 0 });
        }
    }

    fn set_last_in_chain(&mut self, last: bool) {
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
