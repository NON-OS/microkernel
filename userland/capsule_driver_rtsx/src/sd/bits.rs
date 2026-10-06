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

//! A field of a 128-bit register out of a response held as Linux holds it,
//! word 0 the highest: UNSTUFF_BITS (drivers/mmc/core/sd.c).

pub fn unstuff(resp: &[u32; 4], start: u32, size: u32) -> u32 {
    let mask = if size < 32 { (1u32 << size) - 1 } else { u32::MAX };
    let off = 3 - (start / 32) as usize;
    let shift = start & 31;
    let mut value = resp[off] >> shift;
    if size + shift > 32 && off > 0 {
        value |= resp[off - 1] << ((32 - shift) % 32);
    }
    value & mask
}
