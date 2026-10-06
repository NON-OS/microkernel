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

//! The three kernel calls the policy service makes while it answers a frame,
//! as the kernel would answer them: a reply is kept for the test to collect,
//! a push into the kernel's own settings is accepted, and the service lookup
//! knows one trusted setter, `app.settings`, at SETTER_PID. A client's call
//! out (`mk_ipc_call_timeout`) is answered by what the test sets with
//! `answer_calls`, so a proof can put a real service on the other end.
//!
//! The real `nonos_libc` hands pointers to the kernel as integers; this shim
//! is the kernel's side too, so it is where they become memory, under the
//! contract every caller of the real ABI keeps: a pointer names `len` live
//! bytes, or one live value, of the caller's own for the duration of the call.

use std::cell::RefCell;

/// The pid the lookup gives for `app.settings`, the one setter it knows.
pub const SETTER_PID: u32 = 41;

thread_local! {
    static REPLIES: RefCell<Vec<(u32, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
    static SAID: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// Every line written to the serial log since the last call, oldest first.
pub fn take_said() -> Vec<Vec<u8>> {
    SAID.with(|s| core::mem::take(&mut *s.borrow_mut()))
}

/// The serial log, as the kernel keeps it: one line per call.
pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    let line = bytes(buf, len).to_vec();
    SAID.with(|s| s.borrow_mut().push(line));
    len as i64
}

/// Every reply sent since the last call, oldest first, leaving none kept.
pub fn take_replies() -> Vec<(u32, Vec<u8>)> {
    REPLIES.with(|r| core::mem::take(&mut *r.borrow_mut()))
}

fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    // SAFETY: the ABI contract above; the service passes its own buffer.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn store(ptr: *mut u32, value: u32) {
    // SAFETY: the ABI contract above; the output is the caller's local.
    unsafe { *ptr = value }
}

pub fn mk_ipc_reply(dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    let reply = bytes(buf, len).to_vec();
    REPLIES.with(|r| r.borrow_mut().push((dest_pid, reply)));
    len as i64
}

pub fn mk_service_lookup(name: *const u8, name_len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    if bytes(name, name_len) != b"app.settings" {
        return -2;
    }
    store(port, 4728);
    store(pid, SETTER_PID);
    0
}

/// The service's own pid, which the wallpaper catalog opens its vfs stream as.
pub fn mk_getpid() -> u32 {
    SERVICE_PID
}

/// The pid `mk_getpid` answers.
pub const SERVICE_PID: u32 = 7;

/// What answers a call on this thread: the reply to `(endpoint, request,
/// budget in ms)`, or `None` for one whose reply did not come within it.
type Callee = Box<dyn FnMut(u64, &[u8], u64) -> Option<Vec<u8>>>;

thread_local! {
    static CALLEE: RefCell<Option<Callee>> = const { RefCell::new(None) };
}

/// Answer every `mk_ipc_call_timeout` on this thread with `callee`, which
/// stands for the service called and the kernel between.
pub fn answer_calls(callee: impl FnMut(u64, &[u8], u64) -> Option<Vec<u8>> + 'static) {
    CALLEE.with(|c| *c.borrow_mut() = Some(Box::new(callee)));
}

/// What the kernel returns for a call whose reply did not come in time.
pub const ETIMEDOUT: i64 = -110;

/// A call, as the kernel makes it: the request out, then the reply copied
/// into `resp`, cut to its length, or a timeout. The caller may pass one
/// buffer for both, so the request is copied out before the reply goes in.
pub fn mk_ipc_call_timeout(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    let request = bytes(req, req_len).to_vec();
    let reply =
        CALLEE.with(|c| c.borrow_mut().as_mut().and_then(|f| f(endpoint, &request, timeout_ms)));
    let Some(reply) = reply else {
        return ETIMEDOUT;
    };
    let n = reply.len().min(resp_len);
    fill(resp, &reply[..n]);
    n as i64
}

fn fill(ptr: *mut u8, data: &[u8]) {
    // SAFETY: the ABI contract above; `ptr` names at least `data.len()` bytes
    // of the caller's, and nothing borrowed from them is alive here.
    unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len()) }
}

pub fn mk_admin_policy_push(_field_id: u32, _kind: u32, _value: *const u8, _len: u32) -> i64 {
    0
}
