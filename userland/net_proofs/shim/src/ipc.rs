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

//! IPC: calls out to the service the proof plays, and replies kept for it.

use std::sync::Mutex;

mod req;
pub use req::*;

type Responder = Box<dyn FnMut(u64, &[u8]) -> Vec<u8> + Send>;

static RESPONDER: Mutex<Option<Responder>> = Mutex::new(None);
pub(crate) static REPLIES: Mutex<Vec<Reply>> = Mutex::new(Vec::new());

pub(crate) const ENOENT: i64 = -2;

/// One reply the capsule sent: to a pid, or by request id (MkIpcReplyTo), the other 0.
#[derive(Clone, Debug)]
pub struct Reply {
    pub pid: u32,
    pub request: u64,
    pub bytes: Vec<u8>,
}

/// Play the service behind every endpoint the capsule calls. The responder
/// returns the whole reply frame; it is cut to the caller's buffer as the
/// kernel would.
pub fn set_responder(f: impl FnMut(u64, &[u8]) -> Vec<u8> + Send + 'static) {
    *RESPONDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(f));
}

/// Every reply sent since the last call, oldest first.
pub fn take_replies() -> Vec<Reply> {
    core::mem::take(&mut *REPLIES.lock().unwrap_or_else(|e| e.into_inner()))
}

pub(crate) fn input<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() || len == 0 {
        return &[];
    }
    // SAFETY: the capsule passes a pointer and length to one of its own buffers.
    unsafe { core::slice::from_raw_parts(ptr, len) }
}

pub fn mk_ipc_call(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    cap: usize,
) -> i64 {
    let message = input(req, req_len).to_vec();
    let reply = {
        let mut slot = RESPONDER.lock().unwrap_or_else(|e| e.into_inner());
        match slot.as_mut() {
            Some(f) => f(endpoint, &message),
            None => return ENOENT,
        }
    };
    let n = reply.len().min(cap);
    // SAFETY: `resp` is the capsule's buffer of `cap` bytes and `n <= cap`.
    unsafe { core::ptr::copy_nonoverlapping(reply.as_ptr(), resp, n) };
    n as i64
}

pub fn mk_ipc_reply(pid: u32, buf: *const u8, len: usize) -> i64 {
    let bytes = input(buf, len).to_vec();
    REPLIES.lock().unwrap_or_else(|e| e.into_inner()).push(Reply { pid, request: 0, bytes });
    len as i64
}
