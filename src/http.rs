use crate::bindings::{
    NGX_HTTP_OK,
    NGX_HTTP_INTERNAL_SERVER_ERROR,
    ngx_int_t,
    ngx_uint_t,
    ngx_chain_t,
    ngx_pool_t,
    ngx_http_request_t,
    off_t,
    ngx_http_discard_request_body,
    ngx_http_output_filter,
    ngx_http_send_header,
};

use crate::core::Status;

#[macro_export]
macro_rules! extern_http_request_handler {
    ( $x: ident, $y: ident ) => {
        #[no_mangle]
        pub extern fn $x(r: *mut ngx_http_request_t) -> ngx_int_t {
            $y(&mut http::Request::new(r)).0
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

pub struct Request(*mut ngx_http_request_t);

impl Request {
    pub fn new(r: *mut ngx_http_request_t) -> Request {
        Request(r)
    }

    pub fn is_main(&self) -> bool {
        self.0 == unsafe { (*self.0).main }
    }

    pub fn pool(&self) -> *mut ngx_pool_t {
        unsafe { (*self.0).pool }
    }

    pub fn discard_request_body(&mut self) -> Status
    {
        Status(unsafe { ngx_http_discard_request_body(self.0) })
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
