// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's replies (its server/respond.rs), recorded instead of sent,
//! with the frame each would carry.

use std::cell::RefCell;

use crate::protocol::{response_header, write_status, Request, HDR_LEN, STATUS_LEN};

/// One reply: to whom, for which request, its status, its data length, and
/// when on the test clock it was sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reply {
    pub to: u32,
    pub op: u16,
    pub request_id: u32,
    pub status: i32,
    pub data: usize,
    pub at: u64,
}

thread_local! {
    static SENT: RefCell<Vec<Reply>> = const { RefCell::new(Vec::new()) };
}

/// The replies sent since the last look, oldest first.
pub fn sent() -> Vec<Reply> {
    SENT.with(|s| s.take())
}

fn record(to: u32, req: &Request, status: i32, data: usize) -> i64 {
    let (op, request_id, at) = (req.op, req.request_id, crate::clock::at());
    SENT.with(|s| s.borrow_mut().push(Reply { to, op, request_id, status, data, at }));
    0
}

pub fn status(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) -> i64 {
    response_header(tx, req, STATUS_LEN as u32);
    write_status(tx, errno);
    record(sender_pid, req, errno, 0)
}

pub fn payload(sender_pid: u32, req: &Request, body_len: usize, tx: &mut [u8]) -> i64 {
    response_header(tx, req, (STATUS_LEN + body_len) as u32);
    write_status(tx, 0);
    assert!(HDR_LEN + STATUS_LEN + body_len <= tx.len(), "a reply past its buffer");
    record(sender_pid, req, 0, body_len)
}
