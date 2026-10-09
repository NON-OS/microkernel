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

use crate::bits::Bits;
use crate::error::Error;
use crate::lut::{LUT0, LUT1, LUT2};
use crate::metablock::Meta;
use alloc::vec::Vec;

/// One literal, coded by the tree its block type and context pick.
pub(crate) fn literal(b: &mut Bits, m: &mut Meta, out: &mut Vec<u8>) -> Result<(), Error> {
    let t = m.lit.next(b)?;
    let n = out.len();
    let p1 = if n > 0 { out[n - 1] as usize } else { 0 };
    let p2 = if n > 1 { out[n - 2] as usize } else { 0 };
    let ctx = match m.modes[t] {
        0 => p1 & 63,
        1 => p1 >> 2,
        2 => (LUT0[p1] | LUT1[p2]) as usize,
        _ => (LUT2[p1] << 3 | LUT2[p2]) as usize,
    };
    let tree = m.lit_map[64 * t + ctx] as usize;
    out.push(m.lit_codes[tree].read(b)? as u8);
    Ok(())
}
