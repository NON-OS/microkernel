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

//! A Huffman tree description (RFC 8878 4.2.1): the weights of every symbol
//! but the last, written directly as nibbles or compressed with FSE.

use alloc::vec::Vec;

use super::back::Back;
use super::fse_build::build;
use super::fse_read::counts;
use super::limits::{WEIGHT_LOG, WEIGHT_MAX};

/// At most 255 weights are written; the 256th is implied.
const MAX_WEIGHTS: usize = 255;

/// The weights and the bytes the description took.
pub fn weights(d: &[u8]) -> Option<(Vec<u8>, usize)> {
    let head = *d.first()?;
    if head >= 128 {
        let n = usize::from(head - 127);
        let packed = d.get(1..1 + n.div_ceil(2))?;
        let nibble = |i: usize| match i % 2 {
            0 => packed[i / 2] >> 4,
            _ => packed[i / 2] & 0x0F,
        };
        return Some(((0..n).map(nibble).collect(), 1 + n.div_ceil(2)));
    }
    let body = d.get(1..1 + usize::from(head))?;
    Some((compressed(body)?, 1 + usize::from(head)))
}

/// Two FSE states share one backward stream, taking turns; once a state update
/// runs past the start, the other state's symbol is the last.
fn compressed(body: &[u8]) -> Option<Vec<u8>> {
    let (log, norm, used) = counts(body, WEIGHT_LOG, WEIGHT_MAX)?;
    let table = build(log, &norm)?;
    let mut bits = Back::new(body.get(used..)?)?;
    let mut states = [bits.read(log.into()) as usize, bits.read(log.into()) as usize];
    if bits.overrun() {
        return None;
    }
    let mut out = Vec::new();
    for turn in 0.. {
        let (me, other) = (turn % 2, 1 - turn % 2);
        let cell = *table.cells.get(states[me])?;
        out.push(cell.sym);
        states[me] = usize::from(cell.base) + bits.read(cell.bits.into()) as usize;
        if bits.overrun() {
            out.push(table.cells.get(states[other])?.sym);
            break;
        }
        if out.len() > MAX_WEIGHTS {
            return None;
        }
    }
    (out.len() <= MAX_WEIGHTS).then_some(out)
}
