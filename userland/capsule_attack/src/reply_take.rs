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

//! reply-twice's calls to itself and its receives (reply_twice.rs). Every
//! line the capsule prints is also put in this inbox by MkDebug
//! (src/syscall/microkernel/debug/proc_mirror.rs), from itself and owing no
//! reply: request 0. Those are passed over uncounted and unprinted, or the
//! case's own lines would stand between it and its requests.

use nonos_libc::ipc::{mk_ipc_recv_req, IpcReqMeta};
use nonos_libc::mk_ipc_call_timeout;

/// test.attack's service port (src/userspace/capsule_attack/spawn.rs).
pub const OWN_PORT: u64 = 4954;
const ETIMEDOUT: i64 = -110;
const TAKE_MS: u64 = 2_000;
const NO_WAIT: u64 = u64::MAX;
/// More than the inbox holds (1024), so a full inbox of lines is passed.
const LOOKS: usize = 2_048;

/// A call to itself nobody answers in time: its request stays queued.
pub fn unanswered(body: &[u8; 1]) -> bool {
    let mut back = [0u8; 8];
    let rc = mk_ipc_call_timeout(OWN_PORT, body.as_ptr(), 1, back.as_mut_ptr(), back.len(), 50);
    rc == ETIMEDOUT
}

fn receive(wait: u64) -> (i64, u8, IpcReqMeta) {
    let (mut buf, mut meta) = ([0u8; 8], IpcReqMeta::default());
    let rc = mk_ipc_recv_req(0, buf.as_mut_ptr(), buf.len(), wait, &mut meta);
    (rc, buf[0], meta)
}

/// Empty the inbox of what owes no reply: the lines printed so far.
pub fn drain_lines() {
    for _ in 0..LOOKS {
        let (rc, _, meta) = receive(NO_WAIT);
        if rc <= 0 || meta.request != 0 {
            return;
        }
    }
}

/// The id of the next request in its inbox from `from` carrying `body`.
pub fn take(body: u8, from: u32) -> Option<u64> {
    let mut requests = 0;
    for _ in 0..LOOKS {
        let (rc, first, meta) = receive(TAKE_MS);
        if rc > 0 && meta.request == 0 {
            continue;
        }
        if rc == 1 && first == body && meta.sender == from {
            return Some(meta.request);
        }
        crate::reply_seen::seen(rc, first, &meta);
        requests += 1;
        if rc < 0 || requests == 16 {
            return None;
        }
    }
    None
}
