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

//! How the output window gets room and gives it back: each growth asks the
//! allocator first, and the finished output keeps no more than it holds.

use alloc::vec::Vec;

use super::out::Out;

impl Out {
    /// Grows `buf` to `len` zeroed bytes if the allocator can give them.
    pub(super) fn resize(&mut self, len: usize) -> bool {
        let more = len.saturating_sub(self.buf.len());
        if self.buf.try_reserve_exact(more).is_err() {
            self.short = Some(len);
            return false;
        }
        self.buf.resize(len, 0);
        true
    }

    /// The output, its spare room given back: a member sized by another
    /// member's ISIZE would otherwise hold that much while it is kept.
    pub fn finish(mut self) -> Vec<u8> {
        self.buf.truncate(self.pos);
        if self.buf.capacity() > self.pos.saturating_mul(2).max(1 << 16) {
            self.buf.shrink_to_fit();
        }
        self.buf
    }
}
