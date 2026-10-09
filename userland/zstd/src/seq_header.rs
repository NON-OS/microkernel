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

//! The sequences section header: how many sequences, and the table each of
//! the three codes is read with (RFC 8878 3.1.1.3.2.1).

use super::context::Context;
use super::fse_build::{build, Fse};
use super::fse_default::{literal_lengths, match_lengths, offsets};
use super::fse_read::counts;
use super::limits::{LL_LOG, LL_MAX, ML_LOG, ML_MAX, OF_LOG, OF_MAX};

/// The number of sequences, and the bytes the header took.
pub fn header(d: &[u8], ctx: &mut Context) -> Option<(usize, usize)> {
    let byte = |i: usize| d.get(i).map(|&b| usize::from(b));
    let (count, mut at) = match byte(0)? {
        0 => return Some((0, 1)),
        b @ 1..=127 => (b, 1),
        b @ 128..=254 => (((b - 128) << 8) + byte(1)?, 2),
        _ => (byte(1)? + (byte(2)? << 8) + 0x7F00, 3),
    };
    let modes = byte(at)?;
    at += 1;
    if modes & 3 != 0 {
        return None;
    }
    let specs = [
        (modes >> 6, LL_LOG, LL_MAX, 0),
        (modes >> 4, OF_LOG, OF_MAX, 1),
        (modes >> 2, ML_LOG, ML_MAX, 2),
    ];
    for (mode, log, max, which) in specs {
        let slot = match which {
            0 => &mut ctx.ll,
            1 => &mut ctx.of,
            _ => &mut ctx.ml,
        };
        let (table, used) = match mode & 3 {
            0 => ([literal_lengths, offsets, match_lengths][which]()?, 0),
            1 => match byte(at)? {
                sym if sym <= max => (Fse::single(sym as u8), 1),
                _ => return None,
            },
            2 => {
                let (l, norm, used) = counts(d.get(at..)?, log, max)?;
                (build(l, &norm)?, used)
            }
            // Repeat: the table the last block used, which must exist.
            _ => (slot.clone()?, 0),
        };
        *slot = Some(table);
        at += used;
    }
    Some((count, at))
}
