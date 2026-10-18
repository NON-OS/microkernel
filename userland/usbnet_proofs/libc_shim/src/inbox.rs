// NONOS Operating System (AGPL-3.0-or-later)
//! The core's own inbox, where net.core's requests arrive. The proofs
//! call its answer directly, so nothing arrives here and replies go
//! nowhere.

pub fn mk_ipc_recv_from(_ep: u64, _buf: *mut u8, _len: usize, _ms: u64, _from: *mut u32) -> i64 {
    0
}

pub fn mk_ipc_reply(_pid: u32, _buf: *const u8, len: usize) -> i64 {
    len as i64
}
