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

//! Filling one answer with what has come back for a stream.

extern crate alloc;

use alloc::vec::Vec;

use super::inbox::Inbox;

/// What one read of the mixnet brought.
pub enum Received {
    /// Whole messages, for any stream.
    Messages(Vec<Vec<u8>>),
    /// Nothing yet.
    Empty,
    /// The transport is gone.
    Gone,
}

/// Where the messages come from, and the clock the wait is measured on.
pub trait Mixnet {
    /// Everything delivered that one read brings, waiting at most `wait_ms`
    /// on an empty link.
    fn receive(&mut self, wait_ms: u32) -> Received;
    fn now_ms(&self) -> i64;
}

/// The most reads of the mixnet one answer makes. Messages for other streams
/// keep arriving while one is being answered, and without a bound one busy
/// stream could hold every other caller's answer back.
pub const PULLS_MAX: usize = 32;

/// What one answer carries.
pub struct Gathered {
    pub bytes: Vec<u8>,
    /// The far end finished, and these are the last of its bytes.
    pub closed: bool,
    /// The transport went away: nothing more will come on any stream.
    pub gone: bool,
}

/// Bring back what continues `conn`'s stream: at most `room` bytes, waiting
/// at most `hold_ms` when nothing has come, and filing every message read on
/// the way through `file`, whichever stream it belongs to.
///
/// The old loop took one message and answered as soon as it had any bytes,
/// so a TLS flight that had arrived as sixty messages took sixty answers to
/// collect, each a round trip from the browser. This keeps reading while the
/// mixnet has more and the answer has room, and waits only when it has
/// nothing at all to give.
pub fn gather<M: Mixnet>(
    inbox: &mut Inbox,
    conn: u64,
    room: usize,
    hold_ms: i64,
    net: &mut M,
    mut file: impl FnMut(&mut Inbox, &[u8]),
) -> Gathered {
    let deadline = net.now_ms().saturating_add(hold_ms);
    let mut out = Vec::new();
    let mut pulls = 0usize;
    loop {
        let (bytes, closed) = inbox.drain(conn, room.saturating_sub(out.len()));
        out.extend_from_slice(&bytes);
        if closed {
            return Gathered { bytes: out, closed: true, gone: false };
        }
        if out.len() >= room || pulls >= PULLS_MAX {
            break;
        }
        let wait = match out.is_empty() {
            true => deadline.saturating_sub(net.now_ms()).clamp(0, u32::MAX as i64) as u32,
            false => 0,
        };
        pulls += 1;
        match net.receive(wait) {
            Received::Messages(messages) if !messages.is_empty() => {
                for message in &messages {
                    file(inbox, message);
                }
            }
            Received::Messages(_) | Received::Empty => {
                if !out.is_empty() || net.now_ms() >= deadline {
                    break;
                }
            }
            // Nothing more can come; what is in hand still goes.
            Received::Gone => return Gathered { bytes: out, closed: false, gone: true },
        }
    }
    Gathered { bytes: out, closed: false, gone: false }
}
