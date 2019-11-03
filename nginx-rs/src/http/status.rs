use crate::bindings::*;
use crate::core::Status;

pub struct HTTPStatus(pub ngx_uint_t);

impl Into<Status> for HTTPStatus {
    fn into(self) -> Status {
        Status(self.0 as ngx_int_t)
    }
}

impl Into<ngx_uint_t> for HTTPStatus {
    fn into(self) -> ngx_uint_t {
        self.0
    }
}

pub const HTTP_OK: HTTPStatus = HTTPStatus(NGX_HTTP_OK as ngx_uint_t);
pub const HTTP_INTERNAL_SERVER_ERROR: HTTPStatus = HTTPStatus(NGX_HTTP_INTERNAL_SERVER_ERROR as ngx_uint_t);
pub const HTTP_FORBIDDEN: HTTPStatus = HTTPStatus(NGX_HTTP_FORBIDDEN as ngx_uint_t);
