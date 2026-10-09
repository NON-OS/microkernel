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

use alloc::vec::Vec;

use super::types::Entry;

impl Entry {
    /// The oldest received bytes, no more than `max` of them. A longer block
    /// is split and its rest stays at the front of the queue, so one read
    /// never has to carry more than its reply holds and nothing is dropped
    /// or reordered.
    pub fn take_rx(&mut self, max: usize) -> Option<Vec<u8>> {
        if max == 0 {
            return None;
        }
        let mut front = self.rx.pop_front()?;
        if front.len() > max {
            let rest = front.split_off(max);
            self.rx.push_front(rest);
        }
        Some(front)
    }
}
