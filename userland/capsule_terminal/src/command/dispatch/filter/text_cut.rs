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

//! `cut` over piped lines: one delimited field of each.

use alloc::vec::Vec;

use super::flags;

// cut -d <delim> -f <n>: emit the n-th delim-separated field of each line
// (1-based; default delimiter space, default field 1). Missing field -> "".
// The value may follow its flag or be joined to it, -d, or -d ",".
pub fn cut(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let parsed = match flags(b"cut", args) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let delim = parsed.value(b'd').and_then(|d| d.first().copied()).unwrap_or(b' ');
    let field = parsed.value(b'f').map(field_index).unwrap_or(1);
    input
        .into_iter()
        .map(|l| l.split(|&b| b == delim).nth(field - 1).map(<[u8]>::to_vec).unwrap_or_default())
        .collect()
}

fn field_index(digits: &[u8]) -> usize {
    let mut n = 0usize;
    for &c in digits {
        if c.is_ascii_digit() {
            n = n.saturating_mul(10).saturating_add((c - b'0') as usize);
        }
    }
    n.max(1)
}
