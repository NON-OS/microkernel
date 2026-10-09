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

use super::bits::Bools;

/* Multi-bit reads built on the single boolean. */
impl Bools<'_> {
    /// An n-bit unsigned literal, most significant bit first.
    pub(super) fn literal(&mut self, n: u32) -> u32 {
        (0..n).fold(0, |v, _| (v << 1) | self.bit(128) as u32)
    }

    /// An n-bit magnitude followed by its sign.
    pub(super) fn signed(&mut self, n: u32) -> i32 {
        let v = self.literal(n) as i32;
        if self.bit(128) {
            -v
        } else {
            v
        }
    }

    /// An optional signed value: a flag, then `signed(n)` when set.
    pub(super) fn opt_signed(&mut self, n: u32) -> i32 {
        if self.bit(128) {
            self.signed(n)
        } else {
            0
        }
    }
}
