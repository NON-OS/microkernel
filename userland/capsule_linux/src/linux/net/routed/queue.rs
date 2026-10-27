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

//! What a guest wrote to a stream through net.socks5 and the proxy has not
//! taken yet, carried a frame at a time. A frame left unanswered is offered
//! again with the very same bytes, however much was written behind it, so
//! the proxy hands back the answer it kept rather than carrying them twice.
//! Pure: the proofs hold it, and a close's linger with it (linger.rs).

use alloc::vec::Vec;

/// The most a guest's bytes queue before a send waits.
pub const TX_MAX: usize = 64 << 10;

/// The most one frame carries: route link's CARRY_MAX, held to it by the
/// proofs.
pub const FRAME_MAX: usize = 16 << 10;

#[derive(Default)]
pub struct Queue {
    pub(super) bytes: Vec<u8>,
    /// How many of `bytes` the frame still unanswered carries, 0 for none.
    pub(super) offered: usize,
    /// A write failed: nothing more goes, and what was queued is dropped.
    pub(super) broken: bool,
}

impl Queue {
    /// Take the front of `more` as far as there is room: how many bytes.
    pub fn push(&mut self, more: &[u8]) -> usize {
        let n = more.len().min(self.room());
        self.bytes.extend_from_slice(&more[..n]);
        n
    }

    /// What to offer the proxy now: the frame still unanswered, unchanged,
    /// or else the front of the queue; None when there is nothing to send.
    pub fn offer(&mut self) -> Option<&[u8]> {
        if self.broken || self.bytes.is_empty() {
            return None;
        }
        if self.offered == 0 {
            self.offered = self.bytes.len().min(FRAME_MAX);
        }
        Some(&self.bytes[..self.offered])
    }

    /// The proxy answered the frame offered: those bytes have gone.
    pub fn taken(&mut self) {
        self.bytes.drain(..self.offered);
        self.offered = 0;
    }

    /// A write failed: nothing more goes.
    pub fn fail(&mut self) {
        (self.bytes, self.offered, self.broken) = (Vec::new(), 0, true);
    }
}
