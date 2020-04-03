use crate::error::RResult;
use crate::options::RaftOptions;
use crate::options::RaftServerOptions;
use crate::raft::Raft;
use crate::ServerStatus;
use libraft::ffi::root::*;
pub struct RaftServer {
    pub inner: *mut raft_server_cache,
}

impl RaftServer {
    pub fn new(ops: RaftServerOptions) -> Self {
        let server: *mut raft_server_cache = unsafe { raft_server_create(ops.inner) };
        Self { inner: server }
    }

    pub fn start(&self) -> RResult<()> {
        unsafe { raft_server_start(self.inner) };
        Ok(())
    }

    pub fn stop(&self) -> RResult<()> {
        unimplemented!()
    }

    pub fn create_raft(&self, ops: &RaftOptions) -> RResult<Raft> {
        let raft: *mut raft_cache = unsafe { raft_create(self.inner, ops.inner) };

        Ok(Raft { inner: raft })
    }

    pub fn remove_raft(&self, _id: u64) -> RResult<()> {
        unimplemented!()
    }

    pub fn destory_raft(&self, _raft_id: u64, _backup: bool) -> RResult<()> {
        unimplemented!()
    }
    pub fn find_raft(&self, _raft_id: u64) -> RResult<Raft> {
        unimplemented!()
    }

    pub fn get_status(&self) -> RResult<ServerStatus> {
        unimplemented!()
    }

    /// virtual Status SetOptions(const std::map<std::string, std::string>& options) = 0;
    pub fn set_options(&self) -> RResult<()> {
        unimplemented!()
    }

    /// Post a task to all apply threads to run, task should never throw an exception
    /// virtual void PostToAllApplyThreads(const std::function<void()>& task) = 0;
    pub fn post_to_all_apply_threads() {
        unimplemented!()
    }
}
