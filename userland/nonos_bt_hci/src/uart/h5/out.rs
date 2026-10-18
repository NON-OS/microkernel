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

//! The Three-wire UART on one port, sending. Each look sends at most one
//! packet, in this order:
//! 1. an answer to a link control message;
//! 2. a SYNC or CONFIG that is due;
//! 3. once Active, a WAKEUP when the peer is asleep and data waits;
//! 4. the next reliable packet, new or sent again;
//! 5. a bare acknowledgement, when one is owed and nothing else carries it.
//!
//! Every packet carries the acknowledgement number, and once Active the
//! CRC when both sides agreed to it.

use alloc::vec::Vec;

use super::control::WAKEUP;
use super::engine::H5;
use super::header::{Header, ACK, LINK_CONTROL};
use super::link::RESEND_MS;
use super::packet::build;
use super::slip::encode;

impl H5 {
    /// The next octets to write, SLIP framed; `None` when nothing is due.
    pub fn poll_out(&mut self, now: u64, retx_ms: u64) -> Option<Vec<u8>> {
        if let Some(r) = self.replies.pop_front().or_else(|| self.link.due(now)) {
            return self.frame(false, 0, LINK_CONTROL, &r);
        }
        if !self.active() {
            return None;
        }
        if self.peer_asleep && !self.rel.unacked.is_empty() {
            let due = self.woke_at.is_none_or(|t| now.saturating_sub(t) >= RESEND_MS);
            self.woke_at = due.then_some(now).or(self.woke_at);
            return due.then(|| self.frame(false, 0, LINK_CONTROL, &WAKEUP)).flatten();
        }
        if let Some((seq, kind, payload)) = self.rel.next_out(now, retx_ms) {
            return self.frame(true, seq, kind, &payload);
        }
        self.rel.ack_due.then(|| self.frame(false, 0, ACK, &[])).flatten()
    }

    /// Queue an HCI packet as reliable; false while the window is full.
    pub fn send(&mut self, kind: u8, packet: &[u8]) -> bool {
        let room = self.active() && self.rel.room(self.link.agreed.window);
        if room {
            self.rel.queue(kind, packet.to_vec());
        }
        room
    }

    fn frame(&mut self, reliable: bool, seq: u8, kind: u8, payload: &[u8]) -> Option<Vec<u8>> {
        let dic = self.active() && self.link.agreed.crc;
        let ack = self.rel.expect;
        self.rel.ack_due = false;
        let h = Header { seq, ack, dic, reliable, kind, len: 0 };
        Some(encode(&build(h, payload)?, self.deslip.oof))
    }
}
