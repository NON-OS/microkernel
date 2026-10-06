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

//! A frame's blocks, up to and including the one marked last.

use alloc::vec::Vec;

use super::context::Context;
use super::limits::{BLOCK_MAX, MAX_OUT};
use super::literals::literals;
use super::seq_header::header;
use super::seq_run::run;

const HEADER: usize = 3;

/// Decode every block into `out`; `start` is where this frame's output began.
/// Returns the bytes the blocks took.
pub fn blocks(d: &[u8], ctx: &mut Context, out: &mut Vec<u8>, start: usize) -> Option<usize> {
    let mut at = 0usize;
    loop {
        let h = d.get(at..at + HEADER)?;
        let word = u32::from(h[0]) | u32::from(h[1]) << 8 | u32::from(h[2]) << 16;
        let (last, kind, size) = (word & 1 == 1, (word >> 1) & 3, (word >> 3) as usize);
        at += HEADER;
        if size > BLOCK_MAX {
            return None;
        }
        match kind {
            0 => {
                out.extend_from_slice(d.get(at..at + size)?);
                at += size;
            }
            1 => {
                out.resize(out.len() + size, *d.get(at)?);
                at += 1;
            }
            2 => {
                compressed(d.get(at..at + size)?, ctx, out, start)?;
                at += size;
            }
            _ => return None,
        }
        if out.len() > MAX_OUT {
            return None;
        }
        if last {
            return Some(at);
        }
    }
}

fn compressed(d: &[u8], ctx: &mut Context, out: &mut Vec<u8>, start: usize) -> Option<()> {
    let (lits, used) = literals(d, ctx)?;
    let rest = d.get(used..)?;
    let (count, used) = header(rest, ctx)?;
    let body = rest.get(used..)?;
    if count > 0 {
        return run(body, count, ctx, &lits, out, start);
    }
    // No sequences: the literals are the block, and nothing may follow them.
    body.is_empty().then(|| out.extend_from_slice(&lits))
}
