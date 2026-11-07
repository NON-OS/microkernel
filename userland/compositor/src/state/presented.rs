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

//! When the screen last changed: a count of frames that drew and the time
//! the last one was handed to the display, which on a flipping driver is
//! when the flip landed (FLUSH returns after it). What a client's
//! presentation feedback is built from (OP_FRAME_TIME). Pure, so the host
//! proofs hold it.

pub const FRAME_TIME_LEN: usize = 24;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Presented {
    /// Frames that drew and were presented, from 1.
    pub seq: u64,
    /// The kernel's monotonic uptime when the last was, in milliseconds.
    pub at_ms: u64,
}

impl Presented {
    /// A frame drew and the display took it at `now_ms`.
    pub fn shown(&mut self, now_ms: u64) {
        self.seq = self.seq.wrapping_add(1);
        self.at_ms = now_ms;
    }

    /// The answer's body: the count, the time in nanoseconds (millisecond
    /// resolution, as the clock is), and the refresh period, 0 for unknown.
    pub fn encode(&self) -> [u8; FRAME_TIME_LEN] {
        let mut b = [0u8; FRAME_TIME_LEN];
        b[..8].copy_from_slice(&self.seq.to_le_bytes());
        b[8..16].copy_from_slice(&self.at_ms.saturating_mul(1_000_000).to_le_bytes());
        b
    }
}
