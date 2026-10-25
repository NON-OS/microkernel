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

//! The slots the device has not answered, those it answered, and a queue
//! whose oldest frame stays unanswered.

use super::layout::TXQ_ENTRIES;
use super::queue::TxQueue;

/// How long a frame may go unanswered (Linux `IWL_DEF_WD_TIMEOUT`).
pub const TX_STUCK_MS: u64 = 2500;

impl TxQueue {
    /// Frames the device has not answered.
    pub fn in_flight(&self) -> usize {
        self.write.wrapping_sub(self.read) as usize
    }

    /// The device finished every slot before `ssn`: free them. `false` when
    /// that names more slots than are in flight (nothing is freed).
    pub fn reclaim(&mut self, ssn: u16) -> bool {
        let target = ssn as usize & (TXQ_ENTRIES - 1);
        let read = self.read as usize & (TXQ_ENTRIES - 1);
        let n = target.wrapping_sub(read) & (TXQ_ENTRIES - 1);
        if n > self.in_flight() {
            return false;
        }
        self.read = self.read.wrapping_add(n as u16);
        true
    }

    /// Whether the oldest unanswered frame has gone unanswered `TX_STUCK_MS`
    /// at `now_ms`, as far as the looks at this queue show: each look notes
    /// the oldest slot and when it was first seen there. Its slot cannot be
    /// taken back while the device may still read it, so a stuck queue
    /// calls for a firmware restart (Linux's transmit watchdog does too).
    pub fn stuck(&mut self, now_ms: u64) -> bool {
        if self.in_flight() == 0 {
            self.watch = None;
            return false;
        }
        match self.watch {
            Some((read, since)) if read == self.read => now_ms.saturating_sub(since) >= TX_STUCK_MS,
            _ => {
                self.watch = Some((self.read, now_ms));
                false
            }
        }
    }
}
