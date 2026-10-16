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

//! `uniq` over piped lines: collapse runs of equal lines, counted with `-c`.

use alloc::vec::Vec;

use super::flags;
use crate::term::util::format_u64;

/// Collapse runs of equal lines. `-c` prefixes each with how many there were,
/// which is the form this is nearly always reached for: `sort | uniq -c` is
/// how anyone counts anything from a listing.
pub(in super::super) fn uniq(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let count = match flags(b"uniq", args) {
        Ok(p) => p.has(b'c'),
        Err(e) => return e,
    };
    let mut out: Vec<Vec<u8>> = Vec::new();
    let mut runs: Vec<u64> = Vec::new();
    for line in input {
        if out.last().map(|prev| prev == &line).unwrap_or(false) {
            if let Some(n) = runs.last_mut() {
                *n += 1;
            }
            continue;
        }
        out.push(line);
        runs.push(1);
    }
    if !count {
        return out;
    }
    out.into_iter()
        .zip(runs)
        .map(|(line, n)| {
            // Right aligned in a fixed column so the counts form a column of
            // their own and the lines beside them still line up.
            let mut num = [0u8; 24];
            let k = format_u64(n, &mut num);
            let mut row = Vec::with_capacity(6 + 1 + line.len());
            row.resize(6usize.saturating_sub(k), b' ');
            row.extend_from_slice(&num[..k]);
            row.push(b' ');
            row.extend_from_slice(&line);
            row
        })
        .collect()
}
