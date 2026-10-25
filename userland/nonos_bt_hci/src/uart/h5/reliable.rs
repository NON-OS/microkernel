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

//! Reliable packets (Vol 4, Part D, "Reliable Packets"). Each carries a
//! sequence number modulo 8, and every packet carries the number of the
//! next reliable packet its sender expects. A sender keeps at most the
//! agreed window unacknowledged. A receiver takes only the packet it
//! expects, and answers every reliable packet, taken or not, with its
//! acknowledgement number, so a lost one is sent again.
//!
//! An acknowledgement may only cover packets sent and not yet covered: one
//! naming any other number is passed over, not taken as covering packets
//! never sent.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// How long the oldest unacknowledged packet may wait, sent and sent
/// again, with nothing acknowledged before the link counts as lost. It is
/// several link establishment periods, since a peer still finishing
/// establishment takes nothing until its next CONFIG is answered.
pub const LOST_MS: u64 = 1000;

#[derive(Default)]
pub struct Reliable {
    pub(super) next_seq: u8,
    /// Kind and payload of each unacknowledged packet, oldest first.
    pub(super) unacked: VecDeque<(u8, Vec<u8>)>,
    /// How many of them have gone out at least once.
    pub(super) sent: usize,
    /// The next of them to go out in this round.
    pub(super) cursor: usize,
    pub(super) sent_at: Option<u64>,
    /// Since when something has waited with nothing acknowledged.
    pub(super) waiting_since: Option<u64>,
    /// The sequence number of the next reliable packet expected.
    pub expect: u8,
    /// A reliable packet has come since the last acknowledgement went out.
    pub ack_due: bool,
}

impl Reliable {
    pub fn queue(&mut self, kind: u8, payload: Vec<u8>) {
        self.unacked.push_back((kind, payload));
        self.next_seq = (self.next_seq + 1) & 7;
    }

    /// The peer expects `ack` next: drop what that covers.
    pub fn acked(&mut self, ack: u8, now: u64) {
        let oldest = self.next_seq.wrapping_sub(self.unacked.len() as u8) & 7;
        let n = usize::from(ack.wrapping_sub(oldest) & 7);
        if n == 0 || n > self.sent {
            return;
        }
        self.unacked.drain(..n);
        self.sent -= n;
        self.cursor = self.cursor.saturating_sub(n);
        self.sent_at = (self.sent > 0).then_some(now);
        self.waiting_since = (!self.unacked.is_empty()).then_some(now);
    }
}
