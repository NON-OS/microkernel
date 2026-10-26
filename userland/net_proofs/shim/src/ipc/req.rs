// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The capsule's own inbox on the host, and answers by request id.

use std::collections::VecDeque;
use std::sync::Mutex;

use super::{input, Reply, ENOENT, REPLIES};

static INBOX: Mutex<VecDeque<(Vec<u8>, IpcReqMeta)>> = Mutex::new(VecDeque::new());

/// As nonos_userland_libc's, abi/wire.toml [ipc.recv_meta_v1].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IpcReqMeta {
    pub version: u16,
    pub reserved: u16,
    pub sender: u32,
    pub enqueued_ms: u64,
    pub request: u64,
}

/// Nothing arrives at the capsule's own inbox on the host; proofs call its
/// handlers directly.
pub fn mk_ipc_recv_from(_ep: u64, _buf: *mut u8, _len: usize, _ms: u64, _pid: *mut u32) -> i64 {
    0
}

/// Queue a request for `mk_ipc_recv_req` to hand over, with its meta.
pub fn push_request(bytes: &[u8], meta: IpcReqMeta) {
    INBOX.lock().unwrap_or_else(|e| e.into_inner()).push_back((bytes.to_vec(), meta));
}

/// The oldest request `push_request` queued, cut to `len`, or 0 when none.
pub fn mk_ipc_recv_req(_ep: u64, buf: *mut u8, len: usize, _ms: u64, meta: *mut IpcReqMeta) -> i64 {
    let Some((bytes, m)) = INBOX.lock().unwrap_or_else(|e| e.into_inner()).pop_front() else {
        return 0;
    };
    let n = bytes.len().min(len);
    // SAFETY: `buf` is the capsule's buffer of `len` bytes, `n <= len`, and
    // `meta` points at the capsule's own IpcReqMeta.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, n);
        *meta = m;
    }
    n as i64
}

/// Kept like `mk_ipc_reply`, under the request it names; the pid is 0, as
/// the capsule does not say whom it answers.
pub fn mk_ipc_reply_to(request: u64, buf: *const u8, len: usize) -> i64 {
    let bytes = input(buf, len).to_vec();
    REPLIES.lock().unwrap_or_else(|e| e.into_inner()).push(Reply { pid: 0, request, bytes });
    len as i64
}

pub fn mk_service_lookup(_name: *const u8, _len: usize, _port: *mut u32, _pid: *mut u32) -> i64 {
    ENOENT
}
