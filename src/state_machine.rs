use crate::error::*;
use crate::{CmdResult, ConfigChange, RepStatus, Snapshot, StateMachineCallback};
use ::std::ffi::CStr;
use ::std::ptr;
use ::std::slice;
use libc::{self, c_char, c_uchar, c_void};
use libraft::ffi::root::*;

pub unsafe extern "C" fn sm_destructor(_raw_cb: *mut c_void) {
    //TODO
}

pub unsafe extern "C" fn sm_apply(
    raw_cb: *mut c_void,
    data: *const libc::c_char,
    data_size: usize,
    index: u64,
    term: u64,
    rep_status: u16,
    tag: *mut libc::c_void,
) -> u16 {
    let cmd_result = CmdResult {
        data: slice::from_raw_parts::<u8>(data as *const c_uchar, data_size).to_vec(),
        index: index,
        term: term,
        //replicate success or not
        rep_status: RepStatus { code: rep_status },
        //user tag when Propose
        tag: tag,
    };
    let sm = &mut *(raw_cb as *mut StateMachineCallback);
    match sm.target.apply(&cmd_result) {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}
pub unsafe extern "C" fn sm_apply_member_change(
    raw_cb: *mut c_void,
    conf: *const raft_conf_change_t,
    member: u64,
) -> u16 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb.target.apply_member_change(confg_convert(conf), member) {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}

pub fn confg_convert(_conf: *const raft_conf_change_t) -> *const ConfigChange {
    unimplemented!();
}

pub fn snapshot_convert(_snapshot: Snapshot) -> *mut raft_snapshot_t {
    unimplemented!();
}

pub unsafe extern "C" fn sm_persist_applied(raw_cb: *mut c_void) -> u64 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb.target.persist_applied() {
        Ok(rs) => rs,
        Err(_e) => 0,
    }
}
pub unsafe extern "C" fn sm_on_leader_change(raw_cb: *mut c_void, leader: u64, term: u64) {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    cb.target.on_leader_change(leader, term)
}
pub unsafe extern "C" fn sm_get_snapshot(raw_cb: *mut c_void) -> *mut raft_snapshot_t {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb.target.get_snapshot() {
        Ok(sn) => snapshot_convert(sn),
        Err(_e) => ptr::null_mut(),
    }
}

pub unsafe extern "C" fn sm_apply_snapshot_start(
    raw_cb: *mut c_void,
    context: *const c_char,
    data_size: u32,
    index: u64,
) -> u16 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb.target.apply_snapshot_start(
        slice::from_raw_parts(context as *const c_uchar, data_size as usize).to_vec(),
        index,
    ) {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}

pub unsafe extern "C" fn sm_apply_snapshot_data(
    raw_cb: *mut c_void,
    datas: *mut *const c_char,
    data_nums: u32,
    _data_size_list: *mut u32,
) -> u16 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    let data: Vec<Vec<u8>> = slice::from_raw_parts(datas, data_nums as usize)
        .iter()
        .map(|ptr| CStr::from_ptr(*ptr).to_bytes().to_vec())
        .collect();

    match cb.target.apply_snapshot_data(data) {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}
pub unsafe extern "C" fn sm_apply_snapshot_finish(raw_cb: *mut c_void, index: u64) -> u16 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb.target.apply_snapshot_finish(index) {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}

pub unsafe extern "C" fn sm_apply_read_index(
    raw_cb: *mut c_void,
    cmd: *const c_char,
    index: u16,
) -> u16 {
    let cb = &mut *(raw_cb as *mut StateMachineCallback);
    match cb
        .target
        .apply_read_index(CStr::from_ptr(cmd).to_bytes().to_vec(), index)
    {
        Ok(_) => K_OK,
        Err(e) => e.0,
    }
}
