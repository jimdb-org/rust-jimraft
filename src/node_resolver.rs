use crate::NodeResolverCallback;
use ::std::ffi::CString;
use libc::{self, c_char, c_void};

pub unsafe extern "C" fn nr_destructor(_raw_cb: *mut c_void) {
    //TODO
}

pub unsafe extern "C" fn nr_get_node_address(raw_cb: *mut c_void, node_id: u64) -> *mut c_char {
    let sm = &mut *(raw_cb as *mut NodeResolverCallback);
    let ret = match sm.target.get_node_address(node_id) {
        Ok(ip) => ip,
        Err(_e) => String::from("no_ip"),
    };
    CString::new(String::from(ret)).unwrap().into_raw()
}
