mod bindings;
mod core;

#[macro_use]
mod http;

use crate::bindings::*;
use crate::core::*;
use crate::http::*;

use std::ptr;

const HELLO_WORLD: &str = "Hello, world!\n";

extern_http_request_handler!(ngx_http_hello_world_handler, hello_world_handler);

fn hello_world_handler(request: &mut Request) -> Status {
    if !request.discard_request_body().is_ok() {
        return HTTP_INTERNAL_SERVER_ERROR.into();
    }

    request.set_status(HTTP_OK);
    request.set_content_length_n(HELLO_WORLD.len());
    let status = request.send_header();
    if status == ERROR || status > OK || request.set_header_only() {
        return status;
    }

    let mut buf = match calloc_buf(request.pool()) {
        Some(buf) => buf,
        None => return ERROR,
    };

    buf.set_static_str(HELLO_WORLD);
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
