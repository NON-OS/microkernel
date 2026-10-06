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

use super::feed::feed;
use super::reply::{
    progress, ANSWER_MAX, STEP_OPENING_SESSION, STEP_TRYING_ANOTHER_EXIT, STREAM_LOST,
};
use super::request::{ask, lost, Ask};
use super::state::reset;
use super::who::{who, Who, UNNAMED};
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

/// Largest SOCKS exchange worth buffering.
/// Largest exchange worth buffering from a client. A relayed write can be a
/// whole TLS record, so this is sized for the stream rather than for the
/// handshake that opens it.
const RX_MAX: usize = 34 * 1024;

/// Serve SOCKS clients over IPC.
///
/// The client speaks RFC 1928 as bytes; this capsule is the far end of that
/// conversation and the near end of a mixnet tunnel. Nothing here opens a
/// socket, which is what keeps a clearnet path from existing at all.
pub fn run() -> ! {
    reset();
    let mut rx = vec![0u8; RX_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        super::feed::reap_due();
        if n < 0 || sender == 0 {
            continue;
        }
        // A caller with nothing to say is asking whether the far end has
        // answered yet, which is the only way to collect a reply that arrives
        // seconds after the request. It is a request like any other, and the
        // marker is what lets it be sent at all.
        // An unrecognized ask still gets an answer, for the same reason the
        // comment below gives: the caller is blocked on this reply, and a
        // silent continue here makes it wait out its whole timeout and tear
        // the session down for what was one malformed frame. Close it
        // explicitly instead, so the caller fails fast and reconnects.
        // A frame that names a stream is that stream's; one that names none
        // is stream 0, the one conversation every caller had before (`who`).
        let unnamed = who(sender, UNNAMED);
        let out = match ask(&rx[..n as usize]) {
            Some(Ask::Stream(body)) => feed(unnamed, body, ANSWER_MAX).encode(),
            Some(Ask::Numbered(seq, body)) => numbered(unnamed, seq, body),
            Some(Ask::NumberedOn(stream, seq, body)) => numbered(who(sender, stream), seq, body),
            Some(Ask::ResetOn(stream)) => forget(who(sender, stream)),
            Some(Ask::Reset) | None => forget(unnamed),
            Some(Ask::Status) => status(),
        };
        // Every request is answered, including with nothing. A caller blocks
        // on its reply, so staying silent does not mean "no data", it means
        // the caller waits out its whole timeout for an answer already known.
        let _ = mk_ipc_reply(sender, out.as_ptr(), out.len());
    }
}

/// A numbered exchange of `w`'s, by the rule in `kept`.
///
/// One that is not the first of a conversation, for a conversation this
/// proxy does not hold and has no answer kept for, was lost: this proxy was
/// restarted under it, or ended and forgot it. Starting a conversation on
/// it would read the caller's stream bytes as a SOCKS greeting and close,
/// which the caller could only report as the far end hanging up.
fn numbered(w: Who, seq: u32, body: &[u8]) -> Vec<u8> {
    if lost(seq, super::feed::holds(w), super::kept::holds(w, seq)) {
        return Vec::from([STREAM_LOST]);
    }
    super::kept::answer(w, seq, body, |bytes, room| feed(w, bytes, room).encode())
}

/// How far the mixnet has got: ready once a session is open. Asked while
/// this loop runs, net.nym is up; a session is opened by the first CONNECT,
/// and while that takes, this loop is in it and the ask goes unanswered.
fn status() -> Vec<u8> {
    let silent = crate::nym::rotations();
    match crate::nym::trying_another() {
        true => progress(false, STEP_TRYING_ANOTHER_EXIT, silent),
        false => progress(crate::nym::session().is_some(), STEP_OPENING_SESSION, silent),
    }
}

/// Forget `w`'s conversation and the answer kept for it.
fn forget(w: Who) -> Vec<u8> {
    super::kept::forget(w);
    super::feed::reset_client(w).encode()
}
