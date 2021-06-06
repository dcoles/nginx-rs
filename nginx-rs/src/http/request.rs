use crate::{bindings::*, ngx_null_string};
use crate::core::*;

use crate::http::status::*;

use std::os::raw::c_void;

/// Define a static request handler.
///
/// Handlers are expected to take a single [`Request`] argument and return a [`Status`].
#[macro_export]
macro_rules! http_request_handler {
    ( $name: ident, $handler: expr ) => {
        #[no_mangle]
        extern "C" fn $name(r: *mut ngx_http_request_t) -> ngx_int_t {
            let status: Status = $handler(unsafe { &mut $crate::http::Request::from_ngx_http_request(r) });
            status.0
        }
    };
}

pub struct Request(*mut ngx_http_request_t);

impl Request {
    pub unsafe fn from_ngx_http_request(r: *mut ngx_http_request_t) -> Request {
        Request(r)
    }

    pub fn is_main(&self) -> bool {
        self.0 == unsafe { (*self.0).main }
    }

    pub fn pool(&self) -> Pool {
        unsafe { Pool::from_ngx_pool((*self.0).pool) }
    }

    pub fn connection(&self) -> *mut ngx_connection_t {
        unsafe { (*self.0).connection }
    }

    pub fn get_module_loc_conf(&self, module: &ngx_module_t) -> *mut c_void {
        unsafe { *(*self.0).loc_conf.add(module.ctx_index) }
    }

    pub fn get_complex_value(&self, cv: &mut ngx_http_complex_value_t) -> Option<&NgxStr> {
        let mut res = ngx_null_string!();
        unsafe {
            if ngx_http_complex_value(self.0, cv, &mut res) != NGX_OK as ngx_int_t {
                return None;
            }
            Some(NgxStr::from_ngx_str(res))
        }
    }

    pub fn discard_request_body(&mut self) -> Status
    {
        Status(unsafe { ngx_http_discard_request_body(self.0) })
    }

    pub fn user_agent(&self) -> &NgxStr {
        unsafe { NgxStr::from_ngx_str((*(*self.0).headers_in.user_agent).value) }
    }

    pub fn set_status(&mut self, status: HTTPStatus) {
        unsafe {
            (*self.0).headers_out.status = status.into();
        }
    }

    pub fn set_content_length_n(&mut self, n: usize) {
        unsafe {
            (*self.0).headers_out.content_length_n = n as off_t;
        }
    }

    pub fn send_header(&self) -> Status {
        Status(unsafe { ngx_http_send_header(self.0) })
    }

    pub fn set_header_only(&self) -> bool {
        unsafe { (*self.0).header_only() != 0 }
    }

    pub fn output_filter(&mut self, body: &mut ngx_chain_t) -> Status {
        Status(unsafe { ngx_http_output_filter(self.0, body) })
    }
}
