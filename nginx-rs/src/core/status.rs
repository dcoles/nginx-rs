use crate::bindings::*;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Status(pub ngx_int_t);

impl Status {
    pub fn is_ok(&self) -> bool {
        self == &OK
    }
}

impl Into<ngx_int_t> for Status {
    fn into(self) -> ngx_int_t {
        self.0
    }
}

pub const OK: Status = Status(NGX_OK as ngx_int_t);
pub const ERROR: Status = Status(NGX_ERROR as ngx_int_t);
pub const AGAIN: Status = Status(NGX_AGAIN as ngx_int_t);
