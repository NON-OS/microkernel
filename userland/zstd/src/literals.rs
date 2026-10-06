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

//! The literals section of a compressed block (RFC 8878 3.1.1.3.1).

use alloc::vec;
use alloc::vec::Vec;

use super::context::Context;
use super::huff_build::table;
use super::huff_stream::decode;
use super::huff_weights::weights;
use super::limits::BLOCK_MAX;

/// The literals, and the bytes the section took.
pub fn literals(d: &[u8], ctx: &mut Context) -> Option<(Vec<u8>, usize)> {
    let b0 = *d.first()?;
    let byte = |i: usize| d.get(i).map(|&b| usize::from(b));
    let (kind, format) = (b0 & 3, (b0 >> 2) & 3);
    if kind < 2 {
        let (regen, head) = match format {
            0 | 2 => (usize::from(b0 >> 3), 1),
            1 => (usize::from(b0 >> 4) | byte(1)? << 4, 2),
            _ => (usize::from(b0 >> 4) | byte(1)? << 4 | byte(2)? << 12, 3),
        };
        if regen > BLOCK_MAX {
            return None;
        }
        return match kind {
            0 => Some((d.get(head..head + regen)?.to_vec(), head + regen)),
            _ => Some((vec![*d.get(head)?; regen], head + 1)),
        };
    }
    let (head, width) = match format {
        0 | 1 => (3, 10),
        2 => (4, 14),
        _ => (5, 18),
    };
    let v = (0..head).try_fold(0u64, |v, i| Some(v | (byte(i)? as u64) << (8 * i)))?;
    let mask = (1u64 << width) - 1;
    let regen = ((v >> 4) & mask) as usize;
    let comp = ((v >> (4 + width)) & mask) as usize;
    if regen > BLOCK_MAX {
        return None;
    }
    let body = d.get(head..head + comp)?;
    // A fresh tree replaces the last; Treeless reuses it.
    let tree = if kind == 2 {
        let (w, used) = weights(body)?;
        ctx.huff = Some(table(&w)?);
        used
    } else {
        0
    };
    let out = decode(ctx.huff.as_ref()?, body.get(tree..)?, regen, format != 0)?;
    Some((out, head + comp))
}
