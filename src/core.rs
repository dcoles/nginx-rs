use crate::bindings::*;

use std::{slice, ptr};
use std::str;
use std::mem;
use std::os::raw::c_void;
use std::marker::PhantomData;

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
    pub unsafe fn from_ngx_pool(pool: *mut ngx_pool_t) -> Pool {
        assert!(!pool.is_null());
        Pool(pool)
    }

    pub fn create_buffer(&mut self, size: usize) -> Option<TemporaryBuffer> {
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
        let buf = self.calloc_type::<ngx_buf_t>();
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

    pub fn alloc(&mut self, size: usize) -> *mut c_void {
        unsafe { ngx_palloc(self.0, size) }
    }

    pub fn alloc_type<T>(&mut self) -> *mut T {
        self.alloc(mem::size_of::<T>()) as *mut T
    }

    pub fn calloc(&mut self, size: usize) -> *mut c_void {
        unsafe { ngx_pcalloc(self.0, size) }
    }

    pub fn calloc_type<T>(&mut self) -> *mut T {
        self.calloc(mem::size_of::<T>()) as *mut T
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

pub struct NgxStr<'a>(ngx_str_t, PhantomData<&'a [u8]>);

impl<'a> NgxStr<'a> {
    pub fn new(str: &str) -> NgxStr {
        NgxStr(ngx_str_t { len: str.len(), data: str.as_ptr() as *mut u_char }, PhantomData)
    }

    pub unsafe fn from_ngx_str(str: ngx_str_t) -> NgxStr<'a> {
        NgxStr(str, PhantomData)
    }

    pub fn as_bytes(&self) -> &[u8]  {
        unsafe { slice::from_raw_parts(self.0.data, self.0.len) }
    }

    pub fn to_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).unwrap_or_default()
    }

    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }

    pub fn is_empty(&self) -> bool {
        self.0.len == 0
    }
}

impl Default for NgxStr<'_> {
    fn default() -> Self {
        NgxStr(ngx_str_t { len: 0, data: b"".as_ptr() as *mut u_char }, PhantomData)
    }
}
