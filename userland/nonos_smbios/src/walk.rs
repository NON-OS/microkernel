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

//! The structure table, walked with every length checked (DSP0134 6.1): a
//! structure whose length is under its header's four bytes, whose
//! formatted area runs past the table, or whose strings never end, stops
//! the walk there, and what came before stands. The walk also stops after
//! the end-of-table structure (type 127), after `count` structures when
//! the entry point gave a count, and at a bound no real table reaches.

use super::structure::Structure;
use alloc::vec::Vec;

pub const END_OF_TABLE: u8 = 127;
pub const MAX_STRUCTURES: usize = 4096;
const HEADER: usize = 4;

/// The strings from `at`, and where the next structure begins.
fn strings(table: &[u8], at: usize) -> Option<(Vec<&[u8]>, usize)> {
    let rest = table.get(at..)?;
    if rest.starts_with(&[0, 0]) {
        return Some((Vec::new(), at + 2));
    }
    let mut out = Vec::new();
    let mut i = 0;
    loop {
        let len = rest.get(i..)?.iter().position(|b| *b == 0)?;
        out.push(&rest[i..i + len]);
        i += len + 1;
        if *rest.get(i)? == 0 {
            return Some((out, at + i + 1));
        }
    }
}

pub fn structures(table: &[u8], count: Option<u16>) -> Vec<Structure<'_>> {
    let limit = count.map_or(MAX_STRUCTURES, |c| usize::from(c).min(MAX_STRUCTURES));
    let mut out = Vec::new();
    let mut at = 0;
    while out.len() < limit {
        let Some(h) = table.get(at..at + HEADER) else { break };
        let len = usize::from(h[1]);
        let Some(formatted) = table.get(at..at + len).filter(|_| len >= HEADER) else { break };
        let Some((strings, next)) = strings(table, at + len) else { break };
        let kind = h[0];
        out.push(Structure { kind, handle: u16::from_le_bytes([h[2], h[3]]), formatted, strings });
        at = next;
        if kind == END_OF_TABLE {
            break;
        }
    }
    out
}
