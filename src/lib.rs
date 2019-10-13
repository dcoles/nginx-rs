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

struct LocationConf(*mut ngx_http_hello_world_loc_conf_t);

impl LocationConf {
    fn from_request(request: &Request) -> LocationConf {
        LocationConf(unsafe { request.get_module_loc_conf(&ngx_http_hello_world_module) as *mut ngx_http_hello_world_loc_conf_t })
    }

    fn text(&self) -> &ngx_str_t {
        unsafe { &(*self.0).text }
    }
}

extern_http_request_handler!(ngx_http_hello_world_access_handler, access_handler);

fn access_handler(request: &mut Request) -> Status {
    if request.user_agent().to_str().contains("curl") {
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

    let hlcf = LocationConf::from_request(&request);

    // Create body
    let body = format!("Hello, {}!\n", if hlcf.text().is_empty() { request.user_agent().to_str() } else { hlcf.text().to_str() });

    // Send header
    request.set_status(HTTP_OK);
    request.set_content_length_n(body.len());
    let status = request.send_header();
    if status == ERROR || status > OK || request.set_header_only() {
        return status;
    }

    // Send body
    let mut buf = match request.pool().create_buffer_from_str(&body) {
        Some(buf) => buf,
        None => return ERROR,
    };
    assert!(&buf.as_bytes()[..7] == b"Hello, ");
    buf.set_last_buf(request.is_main());
    buf.set_last_in_chain(true);

    let mut out = ngx_chain_t { buf: buf.as_ngx_buf_mut(), next: ptr::null_mut() };
    request.output_filter(&mut out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
