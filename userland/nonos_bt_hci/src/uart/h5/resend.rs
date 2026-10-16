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

//! What the reliable layer sends next, and what it takes in. Packets go
//! out oldest first. When the oldest has waited `retx_ms` with no
//! acknowledgement, every unacknowledged packet goes out again in order.
//! After `LOST_MS` with nothing acknowledged the link counts as lost: a
//! wait that does not end is not one this host keeps.

use alloc::vec::Vec;

use super::reliable::{Reliable, LOST_MS};

impl Reliable {
    pub fn room(&self, window: u8) -> bool {
        self.unacked.len() < usize::from(window)
    }

    pub fn lost(&self, now: u64) -> bool {
        self.waiting_since.is_some_and(|t| now.saturating_sub(t) >= LOST_MS)
    }

    /// The next packet to send as its sequence number, kind and payload.
    pub fn next_out(&mut self, now: u64, retx_ms: u64) -> Option<(u8, u8, Vec<u8>)> {
        let late = self.sent_at.is_some_and(|t| now.saturating_sub(t) >= retx_ms);
        if self.cursor >= self.unacked.len() && late && !self.unacked.is_empty() {
            (self.cursor, self.sent_at) = (0, None);
        }
        let i = self.cursor;
        let (kind, payload) = self.unacked.get(i)?.clone();
        let oldest = self.next_seq.wrapping_sub(self.unacked.len() as u8);
        self.cursor += 1;
        self.sent = self.sent.max(self.cursor);
        self.sent_at.get_or_insert(now);
        self.waiting_since.get_or_insert(now);
        Some((oldest.wrapping_add(i as u8) & 7, kind, payload))
    }

    /// A reliable packet numbered `seq` came: whether it is the one expected.
    pub fn received(&mut self, seq: u8) -> bool {
        self.ack_due = true;
        let ours = seq == self.expect;
        if ours {
            self.expect = (self.expect + 1) & 7;
        }
        ours
    }
}
