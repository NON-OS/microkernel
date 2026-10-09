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

//! The last answer each caller was sent, kept until it asks for the next.
//!
//! An answer carries exit bytes that were taken out of the inbox to build
//! it, and the kernel drops a reply whose caller has already stopped
//! waiting. The browser waits 60 ms on a poll while this server may hold a
//! request up to a second for the exit to answer, so bytes that arrived in
//! between went into a reply nobody received and were gone from the stream.
//! A caller that numbers its exchanges asks again with the same number and
//! gets the same answer; a new number releases the old one.
//!
//! The caller moves to its next number only once an exchange is answered, so
//! whatever it sends after a lost answer comes under the old number, and that
//! is often new bytes: the browser's first write after a CONNECT follows a
//! poll it stopped waiting for, and that write is the TLS hello. Giving the
//! kept answer back for it dropped the hello while the browser counted it
//! sent, so no page over the mixnet got past its first exchange. The kept
//! answer alone goes only to a request with nothing to carry, or with the
//! very bytes already carried; new bytes are carried, and the answer the
//! caller missed goes in front of what they bring back.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::reply::{ANSWER_MAX, STREAM_CLOSED, STREAM_OPEN};
use super::who::{pid_of, Who, STREAMS_PER_CALLER};

struct Kept {
    pid: Who,
    seq: u32,
    carried: Vec<u8>,
    reply: Vec<u8>,
}

static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// The encoded answer to `pid`'s exchange `seq` carrying `body`. `carry`
/// takes bytes to the far end and returns the encoded answer, marker first,
/// holding at most as many stream bytes as it is given room for.
pub fn answer(
    pid: Who,
    seq: u32,
    body: &[u8],
    carry: impl FnOnce(&[u8], usize) -> Vec<u8>,
) -> Vec<u8> {
    let missed = KEPT
        .lock()
        .iter()
        .find(|k| k.pid == pid && k.seq == seq)
        .map(|k| (k.reply.clone(), body.is_empty() || k.carried == body));
    let out = match missed {
        Some((reply, true)) => return reply,
        // A stream that finished has nowhere to take new bytes; the close
        // the caller missed is its answer.
        Some((reply, false)) if reply.first() == Some(&STREAM_CLOSED) => return reply,
        Some((reply, false)) => {
            let earlier = reply.get(1..).unwrap_or_default();
            let now = carry(body, ANSWER_MAX.saturating_sub(earlier.len()));
            let mut out = Vec::with_capacity(earlier.len() + now.len());
            out.push(now.first().copied().unwrap_or(STREAM_OPEN));
            out.extend_from_slice(earlier);
            out.extend_from_slice(now.get(1..).unwrap_or_default());
            out
        }
        None => carry(body, ANSWER_MAX),
    };
    keep(pid, seq, body, &out);
    out
}

/// Keep the answer to `pid`'s exchange `seq`, replacing its previous one.
/// One caller keeps answers for at most as many streams as it may hold, its
/// oldest going first, so naming new streams cannot grow this without bound.
fn keep(pid: Who, seq: u32, carried: &[u8], reply: &[u8]) {
    let mut kept = KEPT.lock();
    kept.retain(|k| k.pid != pid);
    let caller = pid_of(pid);
    if kept.iter().filter(|k| pid_of(k.pid) == caller).count() >= STREAMS_PER_CALLER {
        if let Some(oldest) = kept.iter().position(|k| pid_of(k.pid) == caller) {
            kept.remove(oldest);
        }
    }
    kept.push(Kept { pid, seq, carried: carried.to_vec(), reply: reply.to_vec() });
}

/// Whether an answer to `pid`'s exchange `seq` is kept.
pub fn holds(pid: Who, seq: u32) -> bool {
    KEPT.lock().iter().any(|k| k.pid == pid && k.seq == seq)
}

/// Forget a conversation's kept answer; it is starting over.
pub fn forget(pid: Who) {
    KEPT.lock().retain(|k| k.pid != pid);
}

/// Forget every caller `alive` says has ended: one reply was kept per
/// conversation ever seen, and a caller that ended never asked again.
pub fn forget_ended(alive: impl Fn(u32) -> bool) {
    KEPT.lock().retain(|k| alive(pid_of(k.pid)));
}
