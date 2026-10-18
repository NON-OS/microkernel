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

//! A ring as the device walks it.

/// One ring as the device walks it: where it is, how long, the next slot,
/// and the generation a slot must carry to be the device's.
#[derive(Default, Clone, Copy, Debug)]
pub struct Ring {
    pub base: u64,
    pub size: u32,
    pub next: u32,
    pub gen: u32,
}

impl Ring {
    pub fn at(&self, idx: u32) -> u64 {
        self.base + idx as u64 * 16
    }

    pub fn advance(&mut self) {
        self.next = (self.next + 1) % self.size;
        if self.next == 0 {
            self.gen ^= 1;
        }
    }
}
