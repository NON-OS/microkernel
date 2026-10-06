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

/* Sample `i` of an unfiltered row at bit depth `depth`, as stored. */
pub(super) fn sample(row: &[u8], i: usize, depth: u8) -> u16 {
    match depth {
        16 => u16::from_be_bytes([row[2 * i], row[2 * i + 1]]),
        8 => row[i] as u16,
        d => {
            let bit = i * d as usize;
            let shift = 8 - d as usize - bit % 8;
            ((row[bit / 8] as u16) >> shift) & ((1u16 << d) - 1)
        }
    }
}

/* A stored sample scaled to eight bits: 16-bit keeps the high byte, lower
 * depths stretch their range to 0..255. */
pub(super) fn to8(v: u16, depth: u8) -> u32 {
    match depth {
        16 => (v >> 8) as u32,
        8 => v as u32,
        d => v as u32 * 255 / ((1u32 << d) - 1),
    }
}
