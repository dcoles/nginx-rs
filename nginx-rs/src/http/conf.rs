use crate::bindings::*;

use std::os::raw::c_void;

pub unsafe fn ngx_http_conf_get_module_main_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).main_conf.add(module.ctx_index)
}

pub unsafe fn ngx_http_conf_get_module_srv_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).srv_conf.add(module.ctx_index)
}

pub unsafe fn ngx_http_conf_get_module_loc_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).loc_conf.add(module.ctx_index)
}
