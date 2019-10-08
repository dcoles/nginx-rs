pub mod bindings;
pub mod core;

#[macro_use]
pub mod log;

#[macro_use]
pub mod http;

use crate::bindings::*;
use crate::core::*;
use crate::http::*;

use std::ptr;

extern_http_request_handler!(ngx_http_hello_world_access_handler, access_handler);

fn access_handler(request: &mut Request) -> Status {
    if request.user_agent().contains("curl") {
        return HTTP_FORBIDDEN.into();
    }

    OK
}

extern_http_request_handler!(ngx_http_hello_world_handler, hello_world_handler);

fn hello_world_handler(request: &mut Request) -> Status {
    ngx_log_debug_http!(request, "http hello_world handler");

    // Ignore client request body if any
    if !request.discard_request_body().is_ok() {
        return HTTP_INTERNAL_SERVER_ERROR.into();
    }

    // Create body
    let body = format!("Hello, {}!", request.user_agent());

    // Send header
    request.set_status(HTTP_OK);
    request.set_content_length_n(body.len());
    let status = request.send_header();
    if status == ERROR || status > OK || request.set_header_only() {
        return status;
    }

    // Send body
    let mut buf = match Buffer::create_from_str(request.pool(), &body) {
        Some(buf) => buf,
        None => return ERROR,
    };
    buf.set_last_buf(request.is_main());
    buf.set_last_in_chain(true);

    let mut out = ngx_chain_t { buf: buf.0, next: ptr::null_mut() };
    request.output_filter(&mut out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
