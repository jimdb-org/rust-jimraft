pub mod error;
pub mod node_resolver;
pub mod options;
pub mod raft;
pub mod raft_server;
pub mod state_machine;
pub mod status;
pub use crate::options::*;

pub use crate::{
    error::RResult, error::RaftError, options::RaftOptions, options::RaftServerOptions, raft::Raft,
    raft_server::RaftServer,
};
use libc::{self, c_void};

pub struct ServerStatus {
    //TODO
}

pub enum ConfChangeType {
    ///Add raft member
    KAdd = 0,
    ///Remove raft member
    KRemove = 1,
    ///promote leaner member to normal member
    KPromote = 2,
}

pub struct ConfigChange {
    pub type_: ConfChangeType,
    pub peer: Peer,
    pub context: Vec<u8>,
}

#[derive(Copy, Clone, Debug)]
pub enum PeerType {
    ///normal type
    NORMAL = 0,
    ///learner type
    LEARNER = 1,
}

pub fn conver_to_value(t: PeerType) -> i8 {
    match t {
        PeerType::NORMAL => return 0,
        PeerType::LEARNER => return 1,
    }
}

pub struct Peer {
    pub type_: PeerType,
    pub node_id: u64,
    pub id: u64,
}

pub struct StateMachineCallback {
    pub target: Box<dyn StateMachine>,
}

pub struct NodeResolverCallback {
    pub target: Box<dyn NodeResolver>,
}

pub struct RepStatus {
    pub code: u16,
    // pub msg: String,
}

pub struct CmdResult {
    pub data: Vec<u8>,
    pub index: u64,
    pub term: u64,
    /// Replicate success or not
    pub rep_status: RepStatus,
    /// User tag when Propose
    pub tag: *mut c_void,
}

pub struct Snapshot {
    // TODO
}

pub trait StateMachine {
    fn apply(&mut self, _result: &CmdResult) -> RResult<()>;

    fn apply_member_change(&self, _conf: *const ConfigChange, _member: u64) -> RResult<()>;

    fn persist_applied(&self) -> RResult<u64>;

    fn on_leader_change(&self, leader: u64, term: u64);

    fn get_snapshot(&self) -> RResult<Snapshot>;

    fn apply_snapshot_start(&self, _context: Vec<u8>, index: u64) -> RResult<()>;

    fn apply_snapshot_data(&self, datas: Vec<Vec<u8>>) -> RResult<()>;

    fn apply_snapshot_finish(&mut self, index: u64) -> RResult<()>;

    fn apply_read_index(&self, _cmd: Vec<u8>, index: u16) -> RResult<()>;
}

pub trait NodeResolver {
    fn get_node_address(&self, node_id: u64) -> RResult<String>;
}
