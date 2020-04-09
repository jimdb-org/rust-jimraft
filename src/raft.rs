use crate::error::RResult;
use crate::status::Status;
use ::std::slice;
use libc::{c_char, c_uchar, c_void};
use libraft::ffi::root::*;

pub struct Raft {
    pub inner: *mut raft_cache,
}
unsafe impl Send for Raft {}
unsafe impl Sync for Raft {}

pub struct LogReader {
    pub inner: *mut raft_log_reader_t,
}

impl Raft {
    pub fn is_leader(&self) -> RResult<bool> {
        let is_leader: bool = unsafe { raft_is_leader(self.inner) };
        Ok(is_leader)
    }
    pub fn propose(&self, data: &[u8], flag: u32, tag: *mut c_void) {
        let v = data.as_ref();
        let size = data.len() as usize;
        unsafe { raft_propose(self.inner, v.as_ptr() as *mut c_char, size, flag, tag) };
    }
    pub fn get_leader_term(&self) -> RResult<(u64, u64)> {
        unsafe {
            let tinfo: *mut raft_term_info = raft_get_leader_term(self.inner);
            let leader = raft_get_leader_from_raft_term_info(tinfo);
            let term = raft_get_term_from_raft_term_info(tinfo);
            raft_free_raft_term_info(tinfo);

            return Ok((leader, term));
        };
    }
    pub fn begin_read_log(&self, start_index: u64) -> RResult<LogReader> {
        unsafe {
            let lr: *mut raft_log_reader_t = raft_begin_read_log(self.inner, start_index);
            Ok(LogReader { inner: lr })
        }
    }
}

impl LogReader {
    pub fn next_log(&self) -> RResult<(Status, u64, Vec<u8>, bool)> {
        unsafe {
            let mut index = 0;
            let mut data_: *mut libc::c_char = ::std::ptr::null_mut();
            let mut len: usize = 0;
            let mut over = false;
            let js: *mut jim_status_t =
                raft_log_reader_next(self.inner, &mut index, &mut data_, &mut len, &mut over);

            let data = slice::from_raw_parts::<u8>(data_ as *mut c_uchar, len).to_vec();
            jim_free_jim_status_string(data_); // free c memory
            let status = Status::new(js);
            return Ok((status, index, data, over));
        }
    }
    pub fn last_index(&self) -> RResult<u64> {
        let last_index = unsafe { raft_log_reader_last_index(self.inner) };
        Ok(last_index)
    }
    pub fn end_read_log(&self) -> RResult<()> {
        // free c memory
        unsafe { raft_end_read_log(self.inner) };
        Ok(())
    }
}
