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

//! The counters on the wire: OP_STATS's payload.

use super::stats::Stats;
use super::wire::STATS_LEN;

impl Stats {
    /// The twelve words OP_STATS carries, the last three zero.
    pub fn encode(&self, out: &mut [u8]) -> usize {
        let s = self;
        let head = [s.tx_frames, s.tx_errors, s.rx_frames, s.rx_errors, s.tx_bytes, s.rx_bytes];
        let tail = [s.binds, s.losses, s.failing, 0, 0, 0];
        for (i, w) in head.iter().chain(tail.iter()).enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }
        STATS_LEN
    }
}
