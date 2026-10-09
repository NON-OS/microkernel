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

/// Poly1305 state in radix 2^44: the accumulator `h` and the clamped key
/// `r` as 44, 44 and 42-bit limbs, `pad` the second key half as two words.
pub(crate) struct Poly1305 {
    pub(super) h: [u64; 3],
    pub(super) r: [u64; 3],
    pub(super) pad: [u64; 2],
    pub(super) buffer: [u8; 16],
    pub(super) buffer_len: usize,
}
