use crate::bindings::*;
use crate::core::{Status, Pool};
use std::ffi::c_void;

#[macro_export]
macro_rules! extern_http_request_handler {
    ( $x: ident, $y: ident ) => {
        #[no_mangle]
        pub extern fn $x(r: *mut ngx_http_request_t) -> ngx_int_t {
            $y(&mut $crate::http::Request::from_ngx_http_request(r)).0
        }
    };
}

pub struct HTTPStatus(ngx_uint_t);

impl Into<Status> for HTTPStatus {
    fn into(self) -> Status {
        Status(self.0 as ngx_int_t)
    }
}

pub const HTTP_OK: HTTPStatus = HTTPStatus(NGX_HTTP_OK as ngx_uint_t);
pub const HTTP_INTERNAL_SERVER_ERROR: HTTPStatus = HTTPStatus(NGX_HTTP_INTERNAL_SERVER_ERROR as ngx_uint_t);
pub const HTTP_FORBIDDEN: HTTPStatus = HTTPStatus(NGX_HTTP_FORBIDDEN as ngx_uint_t);

pub struct Request(*mut ngx_http_request_t);

impl Request {
    pub fn from_ngx_http_request(r: *mut ngx_http_request_t) -> Request {
        Request(r)
    }

    pub fn is_main(&self) -> bool {
        self.0 == unsafe { (*self.0).main }
    }

    pub fn pool(&self) -> Pool {
        Pool::from_ngx_pool(unsafe { (*self.0).pool })
    }

    pub fn connection(&self) -> *mut ngx_connection_t {
        unsafe { (*self.0).connection }
    }

    pub fn get_module_loc_conf(&self, module: &ngx_module_t) -> *mut c_void {
        unsafe { *(*self.0).loc_conf.offset(module.ctx_index as isize) }
    }

    pub fn discard_request_body(&mut self) -> Status
    {
        Status(unsafe { ngx_http_discard_request_body(self.0) })
    }

    pub fn user_agent(&mut self) -> &ngx_str_t {
        unsafe { &(*(*self.0).headers_in.user_agent).value }
    }

    pub fn set_status(&mut self, status: HTTPStatus) {
        unsafe {
            (*self.0).headers_out.status = status.0;
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

    pub fn output_filter(&mut self, body: *mut ngx_chain_t) -> Status {
        Status(unsafe { ngx_http_output_filter(self.0, body) })
    }
}
