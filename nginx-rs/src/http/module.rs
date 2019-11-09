use crate::bindings::*;
use crate::core::*;

use std::os::raw::{c_void, c_char};
use core::ptr;

pub trait Merge {
    fn merge(&mut self, prev: &Self);
}

impl Merge for () {
    fn merge(&mut self, _prev: &Self) {}
}

pub trait HTTPModule {
    type MainConf: Merge + Default;
    type SrvConf: Merge + Default;
    type LocConf: Merge + Default;

    unsafe extern "C" fn preconfiguration(_cf: *mut ngx_conf_t) -> ngx_int_t {
        OK.into()
    }

    unsafe extern "C" fn postconfiguration(_cf: *mut ngx_conf_t) -> ngx_int_t {
        OK.into()
    }

    unsafe extern "C" fn create_main_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        pool.allocate::<Self::MainConf>(Default::default()) as *mut c_void
    }

    unsafe extern "C" fn init_main_conf(_cf: *mut ngx_conf_t, _conf: *mut c_void) -> *mut c_char {
        ptr::null_mut()
    }

    unsafe extern "C" fn create_srv_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        pool.allocate::<Self::SrvConf>(Default::default()) as *mut c_void
    }

    unsafe extern "C" fn merge_srv_conf(_cf: *mut ngx_conf_t, prev: *mut c_void, conf: *mut c_void) -> *mut c_char {
        let prev = &mut *(prev as *mut Self::SrvConf);
        let conf = &mut *(conf as *mut Self::SrvConf);
        conf.merge(prev);
        ptr::null_mut()
    }

    unsafe extern "C" fn create_loc_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        pool.allocate::<Self::LocConf>(Default::default()) as *mut c_void
    }

    unsafe extern "C" fn merge_loc_conf(_cf: *mut ngx_conf_t, prev: *mut c_void, conf: *mut c_void) -> *mut c_char {
        let prev = &mut *(prev as *mut Self::LocConf);
        let conf = &mut *(conf as *mut Self::LocConf);
        conf.merge(prev);
        ptr::null_mut()
    }
}
