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

//! A server answers one request twice. The capsule calls its own service
//! port, caller and server both: each call times out, request queued; it
//! takes two of three with MkIpcRecvReq and answers the first twice. The
//! second answer must reach no one, nor one to the third, still queued.

use nonos_libc::ipc::{mk_ipc_recv_req, mk_ipc_reply_to, IpcReqMeta};
use nonos_libc::{mk_getpid, mk_ipc_call_timeout};

use crate::line::{note, verdict};

/// test.attack's service port (src/userspace/capsule_attack/spawn.rs).
const OWN_PORT: u64 = 4954;
/// A receive that looks once (MK_IPC_NO_WAIT), and a call's timeout errno.
const NO_WAIT: u64 = u64::MAX;
const ETIMEDOUT: i64 = -110;

/// A call to itself nobody answers in time: its request stays queued.
fn unanswered(body: &[u8; 1]) -> bool {
    let mut back = [0u8; 8];
    let rc = mk_ipc_call_timeout(OWN_PORT, body.as_ptr(), 1, back.as_mut_ptr(), back.len(), 50);
    rc == ETIMEDOUT
}

/// The id of the next of its own requests in its inbox, carrying `body`.
fn take(body: u8) -> Option<u64> {
    let me = mk_getpid();
    for _ in 0..16 {
        let (mut buf, mut meta) = ([0u8; 8], IpcReqMeta::default());
        let rc = mk_ipc_recv_req(0, buf.as_mut_ptr(), buf.len(), NO_WAIT, &mut meta);
        if rc == 1 && buf[0] == body && meta.sender == me && meta.request != 0 {
            return Some(meta.request);
        }
        crate::reply_seen::seen(rc, buf[0], &meta);
        if rc < 0 {
            return None;
        }
    }
    None
}

pub fn reply_twice() {
    if !(unanswered(b"a") && unanswered(b"b") && unanswered(b"c")) {
        return note(b"[ATTACK-NOTE] reply-twice: a call to itself did not time out queued\n");
    }
    let (Some(a), Some(b)) = (take(b'a'), take(b'b')) else {
        return note(b"[ATTACK-NOTE] reply-twice: its own requests were not in its inbox\n");
    };
    let answer = *b"answer";
    if mk_ipc_reply_to(a, answer.as_ptr(), answer.len()) != 0 {
        return note(b"[ATTACK-NOTE] reply-twice: the first answer was not delivered\n");
    }
    let rc = mk_ipc_reply_to(a, answer.as_ptr(), answer.len());
    verdict(b"reply-twice", b"a second MkIpcReplyTo to one request", rc);
    // Ids run on per server and nothing else calls test.attack: b + 1 is c's.
    let rc = mk_ipc_reply_to(b + 1, answer.as_ptr(), answer.len());
    verdict(b"reply-twice", b"MkIpcReplyTo to a request still in the inbox", rc);
    let _ = mk_ipc_reply_to(b, answer.as_ptr(), answer.len());
    take(b'c').map(|c| mk_ipc_reply_to(c, answer.as_ptr(), answer.len()));
}
