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

    fn text(&self) -> Option<&mut ngx_http_complex_value_t> {
        let text = unsafe { (*self.0).text };
        if text.is_null() {
            None
        } else {
            Some(unsafe { &mut (*text) })
        }
    }
}

extern_http_request_handler!(ngx_http_hello_world_access_handler, access_handler);

fn access_handler(request: &mut Request) -> Status {
    if request.user_agent().as_bytes().starts_with(b"curl") {
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
    let text = match hlcf.text() {
        None => return HTTP_INTERNAL_SERVER_ERROR.into(),
        Some(text) => {
            match request.get_complex_value(text) {
                None => return HTTP_INTERNAL_SERVER_ERROR.into(),
                Some(text) => text
            }
        }
    };
    let user_agent = request.user_agent();
    let body = format!("Hello, {}!\n", if text.is_empty() { user_agent.to_string_lossy() } else { text.to_string_lossy() });

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
        None => return HTTP_INTERNAL_SERVER_ERROR.into(),
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
