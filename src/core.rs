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

pub struct Pool(*mut ngx_pool_t);

impl Pool {
    pub fn from_ngx_pool(pool: *mut ngx_pool_t) -> Pool {
        Pool(pool)
    }

    pub fn create_buffer(&mut self, size: usize) -> Option<TemporaryBuffer> {
        assert!(!self.0.is_null());
        let buf = unsafe { ngx_create_temp_buf(self.0, size) };
        if buf.is_null() {
            return None;
        }

        Some(TemporaryBuffer(buf))
    }

    pub fn create_buffer_from_str(&mut self, str: &str) -> Option<TemporaryBuffer>
    {
        let mut buf = self.create_buffer(str.len())?;
        unsafe {
            ptr::copy_nonoverlapping(str.as_ptr(), (*buf.0).pos, str.len());
            (*buf.0).last = (*buf.0).pos.offset(str.len() as isize);
        }
        Some(buf)
    }

    pub fn create_buffer_from_static_str(&mut self, str: &'static str) -> Option<MemoryBuffer> {
        assert!(!self.0.is_null());
        let buf = unsafe { self.ngx_calloc_buf() };
        if buf.is_null() {
            return None;
        }

        // We cast away const, but buffers with the memory flag are read-only
        let start = str.as_ptr() as *mut u8;
        let end = unsafe { start.offset(str.len() as isize) };

        unsafe {
            (*buf).start = start;
            (*buf).pos = start;
            (*buf).last = end;
            (*buf).end = end;
            (*buf).set_memory(1);
        }

        Some(MemoryBuffer(buf))
    }

    unsafe fn ngx_alloc_buf(&mut self) -> *mut ngx_buf_t {
        ngx_palloc(self.0, mem::size_of::<ngx_buf_t>()) as *mut ngx_buf_t
    }

    unsafe fn ngx_calloc_buf(&mut self) -> *mut ngx_buf_t {
        ngx_pcalloc(self.0, mem::size_of::<ngx_buf_t>()) as *mut ngx_buf_t
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

impl ngx_str_t {
    pub fn as_bytes(&self) -> &[u8]  {
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }

    pub fn to_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).unwrap_or_default()
    }

    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for ngx_str_t {
    fn default() -> Self {
        ngx_str_t { len: 0, data: ptr::null_mut() }
    }
}
