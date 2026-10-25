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

//! One detailed timing descriptor, in the EDID's 18-byte layout.

use crate::tests::wire_sweep::rng::Rng;

/// A detailed timing of `w` x `h` (the EDID's 18-byte layout).
pub fn dtd(r: &mut Rng, w: u32, h: u32) -> [u8; 18] {
    let mut d = [0u8; 18];
    d[0..2].copy_from_slice(&((r.below(0xFFFF) + 1) as u16).to_le_bytes());
    (d[2], d[3], d[4]) =
        (w as u8, r.next() as u8, (((w >> 8) & 0xF) << 4) as u8 | (r.below(16) as u8));
    (d[5], d[6], d[7]) =
        (h as u8, r.next() as u8, (((h >> 8) & 0xF) << 4) as u8 | (r.below(16) as u8));
    d[8..18].iter_mut().for_each(|b| *b = r.next() as u8);
    d
}
