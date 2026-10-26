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

//! What waits for a session's reader, and the session's key wiped on close.

use alloc::vec::Vec;

use super::types::Session;

impl Session {
    pub fn push(&mut self, body: Vec<u8>) {
        let _ = self.rx.push(body);
    }

    /// Messages and bytes waiting for the reader.
    pub fn backlog(&self) -> (usize, usize) {
        (self.rx.len(), self.rx.bytes())
    }

    /// Take at most `limit` bytes of the next message, leaving the rest where
    /// a later read will find it. See `RxQueue::take`.
    pub fn take(&mut self, limit: usize) -> Option<Vec<u8>> {
        self.rx.take(limit)
    }

    /// Move as many queued messages as fit into `out`, as records. See
    /// `RxQueue::fill_records`.
    pub fn fill_records(&mut self, out: &mut [u8]) -> usize {
        self.rx.fill_records(out)
    }

    pub fn zeroize(&mut self) {
        self.key.fill(0);
    }
}
