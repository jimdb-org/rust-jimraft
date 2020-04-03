use serde_json::json;

#[allow(unused)]
pub const K_OK: u16 = 0;
#[allow(unused)]
pub const K_NOT_FOUND: u16 = 1;
#[allow(unused)]
pub const K_CORRUPTION: u16 = 2;
#[allow(unused)]
pub const K_NOT_SUPPORTED: u16 = 3;
#[allow(unused)]
pub const K_INVALID_ARGUMENT: u16 = 4;
#[allow(unused)]
pub const K_IO_ERROR: u16 = 5;
#[allow(unused)]
pub const K_SHUTDOWN_IN_PROGRESS: u16 = 6;
#[allow(unused)]
pub const K_TIMED_OUT: u16 = 7;
#[allow(unused)]
pub const K_ABORTED: u16 = 8;
#[allow(unused)]
pub const K_BUSY: u16 = 9;
#[allow(unused)]
pub const K_EXPIRED: u16 = 10;
#[allow(unused)]
pub const K_DUPLICATED: u16 = 11;
#[allow(unused)]
pub const K_COMPACTED: u16 = 12;
#[allow(unused)]
pub const K_END_OF_FILE: u16 = 13;
#[allow(unused)]
pub const K_NO_LEADER: u16 = 14;
#[allow(unused)]
pub const K_NOT_LEADER: u16 = 15;
#[allow(unused)]
pub const K_STATE_EPOCH: u16 = 16;
#[allow(unused)]
pub const K_EXISTED: u16 = 17;
#[allow(unused)]
pub const K_NO_MEM: u16 = 18;
#[allow(unused)]
pub const K_STALE_RANGE: u16 = 19;
#[allow(unused)]
pub const K_INVALID: u16 = 20;
#[allow(unused)]
pub const K_RESOURCE_EXHAUST: u16 = 21;
#[allow(unused)]
pub const K_NO_LEFT_SPACE: u16 = 22;
#[allow(unused)]
pub const K_UNEXPECTED: u16 = 23;
#[allow(unused)]
pub const K_OUT_OF_BOUND: u16 = 24;
#[allow(unused)]
pub const K_NOT_CHANGE: u16 = 25;
#[allow(unused)]
pub const K_NO_MORE_DATA: u16 = 26;
#[allow(unused)]
pub const K_TYPE_CONFLICT: u16 = 27;
#[allow(unused)]
pub const K_UNKNOWN: u16 = 255;

#[derive(Debug, Clone, PartialEq)]
pub struct RaftError(pub u16, pub String);

impl std::fmt::Display for RaftError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code:{} -> err:[{}]", self.0, self.1)
    }
}

impl std::error::Error for RaftError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl RaftError {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "code": self.0,
            "message": self.1
        })
    }
}

pub fn err_code_str(code: u16, info: &str) -> RaftError {
    RaftError(code, info.to_string())
}
pub fn err_str(info: &str) -> RaftError {
    RaftError(K_UNKNOWN, info.to_string())
}
pub type RResult<T> = std::result::Result<T, RaftError>;
