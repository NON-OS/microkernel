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
//! port: once from itself, a call that gives up (x); once from a worker
//! thread that waits (a, reply_caller.rs); once more after it took a, left
//! queued (z). The answer to a must reach its caller and a second reach no
//! one, nor one to z, still queued, nor one to x, whose call gave up.

use nonos_libc::ipc::mk_ipc_reply_to;
use nonos_libc::mk_getpid;

use crate::line::{note, verdict};
use crate::reply_caller;
use crate::reply_take::{take, unanswered};

pub fn reply_twice() {
    let me = mk_getpid();
    if !unanswered(b"x") {
        return note(b"[ATTACK-NOTE] reply-twice: a call to itself did not time out queued\n");
    }
    let Ok(worker) = reply_caller::start(b'a') else {
        return note(b"[ATTACK-NOTE] reply-twice: no worker thread to call from\n");
    };
    let (Some(x), Some(a)) = (take(b'x', me), take(b'a', worker)) else {
        return note(b"[ATTACK-NOTE] reply-twice: its own requests were not in its inbox\n");
    };
    if !unanswered(b"z") {
        return note(
            b"[ATTACK-NOTE] reply-twice: a call after the worker's did not time out queued\n",
        );
    }
    let answer = *b"Yes";
    let rc = mk_ipc_reply_to(a, answer.as_ptr(), answer.len());
    if rc != 0 || reply_caller::returned() != Some((3, b'Y')) {
        return note(b"[ATTACK-NOTE] reply-twice: the first answer did not reach its caller\n");
    }
    let rc = mk_ipc_reply_to(a, answer.as_ptr(), answer.len());
    verdict(b"reply-twice", b"a second MkIpcReplyTo to one request", rc);
    // Ids run on per server from the call, and only these call test.attack.
    let rc = mk_ipc_reply_to(a + 1, answer.as_ptr(), answer.len());
    verdict(b"reply-twice", b"MkIpcReplyTo to a request still in the inbox", rc);
    let rc = mk_ipc_reply_to(x, answer.as_ptr(), answer.len());
    verdict(b"reply-twice", b"MkIpcReplyTo to a call that gave up", rc);
    let _ = take(b'z', me).map(|z| mk_ipc_reply_to(z, answer.as_ptr(), answer.len()));
}
