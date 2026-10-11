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

use spin::Mutex;

/// Enough words for the 1024 SPIs a frame's ten-bit count can name.
pub(super) const USED_WORDS: usize = 16;

pub(super) struct V2m {
    pub(super) first: u32,
    pub(super) count: u32,
    /// SPIs handed out, a bit each from `first`.
    pub(super) used: [u64; USED_WORDS],
}

pub(super) static V2M: Mutex<Option<V2m>> = Mutex::new(None);

impl V2m {
    pub(super) fn take(&mut self) -> Option<u32> {
        let free = (0..self.count).find(|n| self.used[*n as usize / 64] & 1 << (n % 64) == 0)?;
        self.used[free as usize / 64] |= 1 << (free % 64);
        Some(self.first + free)
    }

    pub(super) fn give(&mut self, intid: u32) {
        let n = intid.wrapping_sub(self.first);
        if n < self.count {
            self.used[n as usize / 64] &= !(1 << (n % 64));
        }
    }
}
