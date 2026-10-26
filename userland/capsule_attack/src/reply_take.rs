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

//! reply-twice's calls to itself and its receives (reply_twice.rs).

use nonos_libc::ipc::{mk_ipc_recv_req, IpcReqMeta};
use nonos_libc::mk_ipc_call_timeout;

/// test.attack's service port (src/userspace/capsule_attack/spawn.rs).
pub const OWN_PORT: u64 = 4954;
const ETIMEDOUT: i64 = -110;
const TAKE_MS: u64 = 2_000;

/// A call to itself nobody answers in time: its request stays queued.
pub fn unanswered(body: &[u8; 1]) -> bool {
    let mut back = [0u8; 8];
    let rc = mk_ipc_call_timeout(OWN_PORT, body.as_ptr(), 1, back.as_mut_ptr(), back.len(), 50);
    rc == ETIMEDOUT
}

/// The id of the next request in its inbox from `from` carrying `body`.
pub fn take(body: u8, from: u32) -> Option<u64> {
    for _ in 0..16 {
        let (mut buf, mut meta) = ([0u8; 8], IpcReqMeta::default());
        let rc = mk_ipc_recv_req(0, buf.as_mut_ptr(), buf.len(), TAKE_MS, &mut meta);
        if rc == 1 && buf[0] == body && meta.sender == from && meta.request != 0 {
            return Some(meta.request);
        }
        crate::reply_seen::seen(rc, buf[0], &meta);
        if rc < 0 {
            return None;
        }
    }
    None
}
