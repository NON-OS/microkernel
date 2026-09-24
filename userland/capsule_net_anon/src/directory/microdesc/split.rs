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

//! Cutting a fetched batch into individual microdescriptors.

extern crate alloc;

use alloc::vec::Vec;

/*
 * A batch is a concatenation with no separator and no count. Each
 * microdescriptor starts with an "onion-key" line at the start of a line, and
 * that is the only boundary there is, which is why the field being optional in
 * the grammar matters here too: a relay that omitted it would merge into its
 * neighbour. Every live relay still publishes one, so the boundary holds, and
 * the digest check on each piece is what would catch it if that changed.
 */
const BOUNDARY: &[u8] = b"onion-key";

/// The byte ranges of each microdescriptor in a fetched batch.
///
pub fn pieces(body: &[u8]) -> Vec<(usize, usize)> {
    let mut starts = Vec::new();
    let mut at = 0usize;
    while at + BOUNDARY.len() <= body.len() {
        let line_start = at == 0 || body.get(at.wrapping_sub(1)) == Some(&b'\n');
        if line_start && &body[at..at + BOUNDARY.len()] == BOUNDARY {
            starts.push(at);
        }
        at += match body[at..].iter().position(|b| *b == b'\n') {
            Some(offset) => offset + 1,
            None => break,
        };
    }
    let mut out = Vec::with_capacity(starts.len());
    for (index, start) in starts.iter().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(body.len());
        out.push((*start, end));
    }
    out
}
