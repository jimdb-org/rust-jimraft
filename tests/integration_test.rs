#[cfg(test)]
mod test_util {
    use jimraft::{error::*, CmdResult, ConfigChange, NodeResolver, Snapshot, StateMachine};
    use std::collections::HashMap;

    pub struct NumberStateMachine {
        pub node_id: u64,
        pub number: u64,
        pub applied: u64,
        pub snaping_number: u64,
    }

    impl StateMachine for NumberStateMachine {
        fn apply(&mut self, _: &CmdResult) -> RResult<()> {
            Ok(())
        }
        fn persist_applied(&self) -> RResult<u64> {
            Ok(self.applied)
        }
        fn apply_member_change(&self, _: *const ConfigChange, _: u64) -> RResult<()> {
            Ok(())
        }
        fn on_leader_change(&self, leader: u64, term: u64) {
            println!("leader changed, leader[{}], term[{}]", leader, term);
        }
        fn get_snapshot(&self) -> RResult<Snapshot> {
            unimplemented!()
        }
        fn apply_snapshot_start(&self, _context: Vec<u8>, _index: u64) -> RResult<()> {
            println!("apply snapshot start");
            Ok(())
        }

        fn apply_snapshot_data(&self, _datas: Vec<Vec<u8>>) -> RResult<()> {
            println!("apply snapshot data");
            Ok(())
        }

        fn apply_snapshot_finish(&mut self, index: u64) -> RResult<()> {
            println!("apply snapshot finish, index[{}]", index);
            Ok(())
        }

        fn apply_read_index(&self, _cmd: Vec<u8>, index: u16) -> RResult<()> {
            println!("apply read index, index[{}]", index);
            Ok(())
        }
    }

    pub struct SimpleNodeResolver {
        pub map: HashMap<u64, &'static str>,
    }

    impl NodeResolver for SimpleNodeResolver {
        fn get_node_address(&self, node_id: u64) -> RResult<String> {
            let ip = self.map.get(&node_id).unwrap();
            Ok(ip.to_string())
        }
    }
    impl SimpleNodeResolver {
        pub fn add_ip(&mut self, node_id: u64, ip: &'static str) {
            if !self.map.contains_key(&node_id) {
                self.map.insert(node_id, ip);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_util::*;
    #[allow(unused_imports)]
    use ::libc::{c_char, c_uint};
    use ::std::collections::HashMap;
    use ::std::ffi::{CStr, CString};
    use ::std::ptr;
    use ::std::{thread, time};
    use jimraft::error::*;
    use jimraft::{
        NodeResolverCallback, Peer, PeerType, Raft, RaftOptions, RaftServer, RaftServerOptions,
        StateMachineCallback,
    };
    use libraft::ffi::root::*;

    // #[test]
    fn jim_status_test() {
        let c = CString::new(String::from("msg1")).unwrap();
        let c_raw: *mut c_char = c.into_raw();

        let c1 = CString::new(String::from("msg2")).unwrap();
        let c1_raw: *mut c_char = c1.into_raw();

        let code = K_NOT_FOUND;

        unsafe {
            let st: *mut jim_status_t = jim_status_create_with_msg(code, c_raw, c1_raw);
            let st_string = jim_status_get_string(st);

            let cstr = CStr::from_ptr(st_string);
            let s = cstr.to_str().unwrap();
            assert_eq!(s, "NotFound: msg1: msg2");

            let c = jim_status_get_code(st);
            assert_eq!(c, K_NOT_FOUND);

            jim_free_jim_status_string(st_string);
            jim_free_jim_status_t(st);
        }

        let code = K_OK;
        unsafe {
            let st: *mut jim_status_t = jim_status_create(code);
            let st_string = jim_status_get_string(st);

            let cstr = CStr::from_ptr(st_string);
            let s = cstr.to_str().unwrap();
            assert_eq!(s, "ok");

            let c = jim_status_get_code(st);
            assert_eq!(c, K_OK);

            jim_free_jim_status_string(st_string);
            jim_free_jim_status_t(st);
        };
    }

    #[test]
    fn test_raft_v2() {
        let tick_interval = 100;
        let election_tick = 10; // 5-10 times of tick_interval
        let sleep_interval = 20; // 20 times of tick_interval

        let server_ops: RaftServerOptions = RaftServerOptions::new();
        server_ops.set_node_id(1);
        server_ops.set_tick_interval(tick_interval);
        server_ops.set_election_tick(election_tick);
        server_ops.set_transport_inprocess_use(true);

        let mut snr = SimpleNodeResolver {
            map: HashMap::new(),
        };
        snr.add_ip(1, "10.8.8.8");
        let nr_callback: NodeResolverCallback = NodeResolverCallback {
            target: Box::new(snr),
        };
        server_ops.set_node_resolver(nr_callback);

        let server: RaftServer = RaftServer::new(server_ops);
        let raft_ops: RaftOptions = RaftOptions::new();
        raft_ops.set_id(1);
        let callback: StateMachineCallback = StateMachineCallback {
            target: Box::new(NumberStateMachine {
                node_id: 1,
                number: 1,
                applied: 1,
                snaping_number: 1,
            }),
        };

        raft_ops.set_state_machine(callback);
        let peers: Vec<Peer> = vec![Peer {
            type_: PeerType::NORMAL,
            node_id: 1,
            id: 1,
        }];
        raft_ops.set_peers(peers);
        raft_ops.set_use_memoray_storage(false);
        let raft: Raft = server.create_raft(&raft_ops).unwrap();
        println!("main:: raft created");

        // must sleep enough time for leader election
        let sleep_millis = time::Duration::from_millis(sleep_interval * tick_interval);

        while !raft.is_leader().unwrap() {
            thread::sleep(sleep_millis);
            println!("waiting fo leader election......");
        }
        println!("{:?}", raft.get_leader_term().unwrap());
        for i in 5..9 {
            // raft.propose(String::from("helllo").as_bytes(), 1, ptr::null_mut());
            raft.propose(&(i as u32).to_be_bytes(), 1, ptr::null_mut());
        }
        match raft.begin_read_log(1) {
            Ok(log) => {
                println!("prepare to read log.......");
                loop {
                    match log.next_log() {
                        Ok((status, index, data, over)) => {
                            if over {
                                break;
                            }
                            println!("data aaaaaaa：{:?}", data);
                            //let d = String::from_utf8(data).unwrap();
                            let mut v: [u8; 4] = Default::default();
                            v.copy_from_slice(&data);
                            let d = u32::from_be_bytes(v);
                            println!(
                                "status: {},data:{}, index: {}, over: {}",
                                status.code, d, index, over
                            );
                        }
                        Err(e) => {
                            println!("error {}", e.to_string());
                            break;
                        }
                    }
                }
                let _rs = log.end_read_log();
            }
            Err(e) => println!("read log error.{}", e.to_string()),
        }
    }
}
