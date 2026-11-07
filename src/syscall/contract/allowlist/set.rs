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

use crate::syscall::abi::index::COUNT;

/// 64-bit words in each bitmap: one bit per registry slot.
pub const WORDS: usize = COUNT.div_ceil(64);

/*
 * `plain` answers for a caller that may not launch apps on another's
 * behalf, `launcher` for one that may (the shell, or a capsule a listing
 * pairs with one). The capability tables consult that one runtime fact;
 * keeping both answers makes the bitmaps exact for any table shape.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Allowlist {
    plain: [u64; WORDS],
    launcher: [u64; WORDS],
}

impl Allowlist {
    /// Permits nothing: the list of a token that failed its checks.
    pub const EMPTY: Self = Self { plain: [0; WORDS], launcher: [0; WORDS] };

    pub fn set(&mut self, slot: usize, plain: bool, launcher: bool) {
        if slot >= COUNT {
            return;
        }
        let (w, b) = (slot / 64, 1u64 << (slot % 64));
        if plain {
            self.plain[w] |= b;
        }
        if launcher {
            self.launcher[w] |= b;
        }
    }

    #[inline]
    pub fn plain(&self, slot: usize) -> bool {
        slot < COUNT && self.plain[slot / 64] & (1u64 << (slot % 64)) != 0
    }

    #[inline]
    pub fn launcher(&self, slot: usize) -> bool {
        slot < COUNT && self.launcher[slot / 64] & (1u64 << (slot % 64)) != 0
    }
}
