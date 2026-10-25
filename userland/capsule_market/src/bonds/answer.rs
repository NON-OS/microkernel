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

//! Settle's answers to the bond cache's reads, taken before any request:
//! believed only from settle's registry pid, with the NSTL magic and the
//! request id in flight.

use nonos_settle_proto::{async_answer, parse_header, parse_read_reply, ReplyError, OP_READ};

use super::wire::{settle, Bonds};

impl Bonds {
    /// Whether `rx` was settle's frame, taken here and needing no reply.
    pub fn on_frame(&mut self, sender: u32, rx: &[u8], now_ms: u64) -> bool {
        if parse_header(rx).is_none() || sender == 0 || settle().map(|s| s.1) != Some(sender) {
            return false;
        }
        let Some((req, pid)) = self.pending else { return true };
        let words = match async_answer(rx, sender, pid, OP_READ, req) {
            Err(ReplyError::Stale | ReplyError::Foreign) => return true,
            Ok(body) => parse_read_reply(body),
            Err(ReplyError::Status(_)) => None,
        };
        self.pending = None;
        let ask = self.cache.on_answer(words, now_ms);
        self.send(ask, now_ms);
        true
    }
}
