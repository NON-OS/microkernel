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

//! The slots the device has not answered, and those it answered.

use super::layout::TXQ_ENTRIES;
use super::queue::TxQueue;

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
}
