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

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

struct Kept {
    pid: u32,
    seq: u32,
    reply: Vec<u8>,
}

static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// The encoded answer already given to `pid` for exchange `seq`, if that is
/// the exchange being asked again.
pub fn again(pid: u32, seq: u32) -> Option<Vec<u8>> {
    KEPT.lock().iter().find(|k| k.pid == pid && k.seq == seq).map(|k| k.reply.clone())
}

/// Keep the answer to `pid`'s exchange `seq`, replacing its previous one.
pub fn keep(pid: u32, seq: u32, reply: &[u8]) {
    let mut kept = KEPT.lock();
    kept.retain(|k| k.pid != pid);
    kept.push(Kept { pid, seq, reply: reply.to_vec() });
}

/// Forget a caller's kept answer; its conversation is starting over.
pub fn forget(pid: u32) {
    KEPT.lock().retain(|k| k.pid != pid);
}
