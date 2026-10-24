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

//! Delivery frames built byte by byte, and a seeded source of noise.

use crate::wire::NINP_MAGIC;

pub(super) fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

pub(super) fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

pub(super) fn delivery(kind: u16, code: u32, x: i32, y: i32) -> Vec<u8> {
    let mut f = NINP_MAGIC.to_le_bytes().to_vec();
    f.extend_from_slice(&[1, 0, 0, 0]);
    f.extend_from_slice(&kind.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&code.to_le_bytes());
    f.extend_from_slice(&x.to_le_bytes());
    f.extend_from_slice(&y.to_le_bytes());
    f.extend_from_slice(&[0; 16]);
    f
}
