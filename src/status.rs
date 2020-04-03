use ::std::ffi::CStr;
use libraft::ffi::root::*;

pub struct Status {
    pub code: u16,
    pub state: &'static str,
}

impl Status {
    pub fn new(js: *mut jim_status_t) -> Status {
        unsafe {
            let code_ = jim_status_get_code(js);
            let state_ = jim_status_get_string(js);
            let cstr = CStr::from_ptr(state_);
            let s = cstr.to_str().unwrap();

            // free c memory
            jim_free_jim_status_string(state_);
            jim_free_jim_status_t(js);

            Status {
                code: code_,
                state: s,
            }
        }
    }
}
