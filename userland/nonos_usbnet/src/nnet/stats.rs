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

//! What the driver counts, for OP_STATS and for noticing a device gone.

use super::wire::STATS_LEN;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Stats {
    pub tx_frames: u32,
    pub tx_errors: u32,
    pub rx_frames: u32,
    pub rx_errors: u32,
    pub tx_bytes: u32,
    pub rx_bytes: u32,
    /// Devices bound, and devices given up as gone, since start.
    pub binds: u32,
    pub losses: u32,
    /// Transfers failed in a row; one that succeeds clears it.
    pub failing: u32,
}

impl Stats {
    pub fn tx(&mut self, sent: Result<usize, i32>) {
        let Ok(n) = sent else { return self.failed(true) };
        self.tx_frames = self.tx_frames.wrapping_add(1);
        self.tx_bytes = self.tx_bytes.wrapping_add(n as u32);
        self.failing = 0;
    }

    pub fn rx(&mut self, got: Result<Option<usize>, i32>) {
        let Ok(got) = got else { return self.failed(false) };
        if let Some(n) = got {
            self.rx_frames = self.rx_frames.wrapping_add(1);
            self.rx_bytes = self.rx_bytes.wrapping_add(n as u32);
        }
        self.failing = 0;
    }

    fn failed(&mut self, tx: bool) {
        let count = if tx { &mut self.tx_errors } else { &mut self.rx_errors };
        *count = count.wrapping_add(1);
        self.failing = self.failing.saturating_add(1);
    }

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
