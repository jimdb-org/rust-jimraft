use crate::node_resolver::{nr_destructor, nr_get_node_address};
use crate::state_machine::{
    sm_apply, sm_apply_member_change, sm_apply_read_index, sm_apply_snapshot_data,
    sm_apply_snapshot_finish, sm_apply_snapshot_start, sm_destructor, sm_get_snapshot,
    sm_on_leader_change, sm_persist_applied,
};

use crate::{conver_to_value, NodeResolverCallback, Peer, StateMachineCallback};
use ::std::ffi::CString;
use ::std::mem;
use libraft::ffi::root::*;

pub struct RaftOptions {
    pub inner: *mut raft_options_t,
}

pub struct RaftServerOptions {
    pub inner: *mut raft_server_options_t,
}

impl RaftServerOptions {
    pub fn new() -> Self {
        let ops: *mut raft_server_options_t = unsafe { raft_server_options_create() };
        Self { inner: ops }
    }

    pub fn set_node_id(&self, node_id: u64) {
        unsafe { raft_server_options_set_node_id(self.inner, node_id) };
    }

    pub fn set_tick_interval(&self, tick_interval: u64) {
        unsafe { raft_server_options_set_tick_interval(self.inner, tick_interval) };
    }

    pub fn set_election_tick(&self, election_tick: u64) {
        unsafe { raft_server_options_set_election_tick(self.inner, election_tick) };
    }

    pub fn set_transport_inprocess_use(&self, flag: bool) {
        unsafe { raft_server_options_set_use_inprocess_transport(self.inner, flag) };
    }

    pub fn set_listen_ip(&self, ip: &str) {
        let ip_ = CString::new(String::from(ip)).unwrap().into_raw();
        unsafe { raft_server_options_set_listen_ip(self.inner, ip_) };
    }

    pub fn set_listen_port(&self, port: u16) {
        unsafe { raft_server_options_set_listen_port(self.inner, port) };
    }

    pub fn set_send_io_threads(&self, num: usize) {
        unsafe { raft_server_options_set_send_io_threads(self.inner, num) };
    }

    pub fn set_recv_io_threads(&self, num: usize) {
        unsafe { raft_server_options_set_recv_io_threads(self.inner, num) };
    }

    pub fn set_connection_pool_size(&self, size: usize) {
        unsafe { raft_server_options_set_connection_pool_size(self.inner, size) };
    }

    pub fn set_node_resolver(&self, callback: NodeResolverCallback) {
        let node_resolver: *mut raft_node_resolver_t = unsafe {
            let cb = Box::new(callback);
            raft_node_resolver_create(
                mem::transmute(cb),
                Some(nr_destructor),
                Some(nr_get_node_address),
            )
        };
        unsafe { raft_server_options_set_node_resolver(self.inner, node_resolver) };
    }
}

impl RaftOptions {
    pub fn new() -> Self {
        let ops: *mut raft_options_t = unsafe { raft_options_create() };
        Self { inner: ops }
    }

    pub fn set_id(&self, id: u64) {
        unsafe { raft_options_set_id(self.inner, id) };
    }

    pub fn set_peers(&self, peers: Vec<Peer>) {
        unsafe {
            let peers_t: *mut raft_peers_t = raft_peers_create();
            for peer in peers.iter() {
                raft_add_peers(peers_t, conver_to_value(peer.type_), peer.node_id, peer.id);
            }
            raft_options_set_peer(self.inner, peers_t);
        }
    }

    pub fn set_use_memoray_storage(&self, flag: bool) {
        unsafe { raft_options_use_memory_storage(self.inner, flag) };
    }

    pub fn set_state_machine(&self, callback: StateMachineCallback) {
        let statemachine: *mut raft_state_machine_t = unsafe {
            let cb = Box::new(callback);
            raft_state_machine_create(
                mem::transmute(cb),
                Some(sm_destructor),
                Some(sm_apply),
                Some(sm_apply_member_change),
                Some(sm_apply_read_index),
                Some(sm_persist_applied),
                Some(sm_on_leader_change),
                Some(sm_get_snapshot),
                Some(sm_apply_snapshot_start),
                Some(sm_apply_snapshot_data),
                Some(sm_apply_snapshot_finish),
            )
        };
        unsafe { raft_options_set_state_machine(self.inner, statemachine) };
    }
}
