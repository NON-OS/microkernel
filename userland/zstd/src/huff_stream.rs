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

//! Huffman-coded literals, in one stream or in four behind a jump table.

use alloc::vec::Vec;

use super::back::Back;
use super::huff_build::Huff;

const JUMP: usize = 6;

pub fn decode(h: &Huff, d: &[u8], regen: usize, four: bool) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(regen);
    if !four {
        one(h, d, regen, &mut out)?;
        return Some(out);
    }
    let size = |i: usize| usize::from(u16::from_le_bytes([d[i], d[i + 1]]));
    d.get(..JUMP)?;
    let (s1, s2, s3) = (size(0), size(2), size(4));
    let s4 = d.len().checked_sub(JUMP)?.checked_sub(s1 + s2 + s3)?;
    let seg = regen.div_ceil(4);
    let last = regen.checked_sub(3 * seg)?;
    let mut at = JUMP;
    for (len, n) in [(s1, seg), (s2, seg), (s3, seg), (s4, last)] {
        one(h, d.get(at..at + len)?, n, &mut out)?;
        at += len;
    }
    Some(out)
}

/// A stream must yield exactly `n` symbols and end on its last bit.
fn one(h: &Huff, d: &[u8], n: usize, out: &mut Vec<u8>) -> Option<()> {
    let mut bits = Back::new(d)?;
    for _ in 0..n {
        let &(sym, len) = h.cells.get(bits.peek(h.max.into()) as usize)?;
        bits.read(len.into());
        if bits.overrun() {
            return None;
        }
        out.push(sym);
    }
    bits.done().then_some(())
}
