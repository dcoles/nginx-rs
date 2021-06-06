use nginx_rs::bindings::*;
use nginx_rs::core::*;
use nginx_rs::http::*;

use nginx_rs::{ngx_modules, ngx_string, http_request_handler, ngx_null_command, ngx_log_debug_http};

use std::borrow::Cow;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[no_mangle]
static mut ngx_http_hello_world_commands: [ngx_command_t; 3] = [
    ngx_command_t {
        name: ngx_string!("hello_world"),
        type_: (NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS) as ngx_uint_t,
        set: Some(ngx_http_hello_world),
        conf: 0,
        offset: 0,
        post: ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("hello_world_text"),
        type_: (NGX_HTTP_LOC_CONF|NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_hello_world_set_text),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: ptr::null_mut(),
    },
    ngx_null_command!(),
];

#[no_mangle]
static ngx_http_hello_world_module_ctx: ngx_http_module_t = ngx_http_module_t {
    preconfiguration: Some(Module::preconfiguration),
    postconfiguration: Some(Module::postconfiguration),

    create_main_conf: Some(Module::create_main_conf),
    init_main_conf: Some(Module::init_main_conf),

    create_srv_conf: Some(Module::create_srv_conf),
    merge_srv_conf: Some(Module::merge_srv_conf),

    create_loc_conf: Some(Module::create_loc_conf),
    merge_loc_conf: Some(Module::merge_loc_conf),
};

#[no_mangle]
pub static mut ngx_http_hello_world_module: ngx_module_t = ngx_module_t {
    ctx_index: ngx_uint_t::max_value(),
    index: ngx_uint_t::max_value(),
    name: ptr::null_mut(),
    spare0: 0,
    spare1: 0,
    version: nginx_version as ngx_uint_t,
    signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

    ctx: &ngx_http_hello_world_module_ctx as *const _ as *mut _,
    commands: unsafe { &ngx_http_hello_world_commands[0] as *const _ as *mut _ },
    type_: NGX_HTTP_MODULE as ngx_uint_t,

    init_master: None,
    init_module: None,
    init_process: None,
    init_thread: None,
    exit_thread: None,
    exit_process: None,
    exit_master: None,

    spare_hook0: 0,
    spare_hook1: 0,
    spare_hook2: 0,
    spare_hook3: 0,
    spare_hook4: 0,
    spare_hook5: 0,
    spare_hook6: 0,
    spare_hook7: 0,
};

ngx_modules!(ngx_http_hello_world_module);

struct Module;

impl HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = LocConf;

    unsafe extern "C" fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        let cmcf = ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module) as *mut ngx_http_core_main_conf_t;

        let h = ngx_array_push(&mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers) as *mut ngx_http_handler_pt;
        if h.is_null() {
            return ERROR.into();
        }

        *h = Some(ngx_http_hello_world_access_handler);

        OK.into()
    }
}

#[derive(Default)]
struct LocConf {
    text: String,
}

impl Merge for LocConf {
    fn merge(&mut self, prev: &LocConf) {
        if self.text.is_empty() {
            self.text = String::from(if !prev.text.is_empty() { &prev.text } else { "" });
        }
    }
}

#[no_mangle]
unsafe extern "C" fn ngx_http_hello_world(cf: *mut ngx_conf_t, _cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
    let conf = &mut *(conf as *mut LocConf);
    let clcf = ngx_http_conf_get_module_loc_conf(cf, &ngx_http_core_module) as *mut ngx_http_core_loc_conf_t;
    (*clcf).handler = Some(ngx_http_hello_world_handler);

    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn ngx_http_hello_world_set_text(cf: *mut ngx_conf_t, _cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
    let conf = &mut *(conf as *mut LocConf);
    let args = (*(*cf).args).elts as *mut ngx_str_t;
    let value = NgxStr::from_ngx_str(*args.add(1));
    conf.text = String::from(value.to_string_lossy());

    ptr::null_mut()
}


http_request_handler!(ngx_http_hello_world_access_handler, |request: &mut Request| {
    if request.user_agent().as_bytes().starts_with(b"curl") {
        return HTTP_FORBIDDEN.into();
    }

    OK
});

http_request_handler!(ngx_http_hello_world_handler, |request: &mut Request| {
    ngx_log_debug_http!(request, "http hello_world handler");

    // Ignore client request body if any
    if !request.discard_request_body().is_ok() {
        return HTTP_INTERNAL_SERVER_ERROR.into();
    }

    let hlcf = unsafe { request.get_module_loc_conf(&ngx_http_hello_world_module) as *mut LocConf };
    let text = unsafe { &(*hlcf).text };

    // Create body
    let user_agent = request.user_agent();
    let body = format!("Hello, {}!\n", if text.is_empty() { user_agent.to_string_lossy() } else { Cow::from(text) });

    // Send header
    request.set_status(HTTP_OK);
    request.set_content_length_n(body.len());
    let status = request.send_header();
    if status == ERROR || status > OK || request.header_only() {
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
});
