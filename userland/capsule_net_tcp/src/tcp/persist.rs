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
//! Two rules about windows that nothing sent enforced.
//!
//! A peer that closes its window opens it again with an ACK of nothing new,
//! and that ACK can be lost. With data queued and nothing in flight, no
//! timer was running and no ACK was owed, so the connection waited forever.
//! The persist timer probes the closed window (RFC 9293 3.8.6.1).
//!
//! A reader that drains the receive queue makes room the peer is not told
//! about until the next segment needs an ACK, and a peer that saw a closed
//! window sends none. The room is announced once it is worth a segment
//! (RFC 9293 3.8.6.2.2).

use super::constants::RTO_MAX_MS;

/// The longest a probe waits, the retransmission timer's ceiling.
pub const PERSIST_MAX_MS: u64 = RTO_MAX_MS as u64;

/// Where the persist timer stands for one connection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Persist {
    /// When the next probe goes out; zero when the timer is not running.
    pub due_ms: u64,
    /// The wait before the probe at `due_ms`.
    pub wait_ms: u64,
    /// Probes sent since the peer last answered.
    pub unanswered: u8,
}

impl Persist {
    pub const IDLE: Persist = Persist { due_ms: 0, wait_ms: 0, unanswered: 0 };
}

/// Whether a closed window holds back queued data with nothing in flight,
/// so no ACK is owed that would reopen it.
pub fn stalled(snd_wnd: u16, snd_una: u32, snd_nxt: u32, queued: usize) -> bool {
    snd_wnd == 0 && snd_una == snd_nxt && queued > 0
}

/// The wait before the next probe: the retransmission timeout for the
/// first, then double the last, never past `PERSIST_MAX_MS`.
pub fn next_wait(last_ms: u64, rto_ms: u64) -> u64 {
    let first = rto_ms.clamp(1, PERSIST_MAX_MS);
    match last_ms {
        0 => first,
        last => last.saturating_mul(2).min(PERSIST_MAX_MS),
    }
}

/// The sequence number a probe carries: one before the oldest unacknowledged
/// byte, which the peer has had, so it answers with an ACK and its window
/// and takes nothing.
pub fn probe_seq(snd_una: u32) -> u32 {
    snd_una.wrapping_sub(1)
}

/// Whether the room a reader made is worth a window update: the window grew
/// since it was last advertised by a full segment or by half the buffer,
/// whichever is smaller, and at least doubled, as Linux asks, so a reader
/// keeping up with a steady stream sends no ACK beside the stream's own.
pub fn worth_announcing(advertised: u16, now: u16, mss: usize, buffer: u16) -> bool {
    let step = (mss as u32).min(u32::from(buffer) / 2).max(1);
    let (advertised, now) = (u32::from(advertised), u32::from(now));
    now > advertised && now - advertised >= step && now >= advertised.saturating_mul(2)
}
