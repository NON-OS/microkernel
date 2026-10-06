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

//! A literal: eight bits under the probabilities its position and the byte
//! before it choose, matched against the byte at rep0 after a match.

use alloc::vec::Vec;

use super::lzma_model::Model;
use super::range::Range;

/// `start` is where the dictionary was last reset; positions count from it.
pub fn literal(rc: &mut Range, m: &mut Model, out: &mut Vec<u8>, start: usize) -> Option<()> {
    let total = out.len() - start;
    let prev = if total > 0 { usize::from(out[out.len() - 1]) } else { 0 };
    let state = ((total & ((1 << m.lp) - 1)) << m.lc) + (prev >> (8 - m.lc));
    let probs = m.literal.get_mut(0x300 * state..0x300 * (state + 1))?;
    let mut sym = 1usize;
    if m.state >= 7 {
        let back = (m.rep[0] as usize).checked_add(1).filter(|&d| d <= total)?;
        let mut byte = u32::from(out[out.len() - back]);
        while sym < 0x100 {
            let bit_m = (byte >> 7) & 1;
            byte <<= 1;
            let bit = rc.bit(&mut probs[((1 + bit_m as usize) << 8) + sym]);
            sym = sym << 1 | bit as usize;
            if bit != bit_m {
                break;
            }
        }
    }
    while sym < 0x100 {
        sym = sym << 1 | rc.bit(&mut probs[sym]) as usize;
    }
    out.push((sym - 0x100) as u8);
    m.state = match m.state {
        0..=3 => 0,
        4..=9 => m.state - 3,
        _ => m.state - 6,
    };
    Some(())
}
