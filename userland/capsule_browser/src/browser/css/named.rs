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

use core::cmp::Ordering;

mod table_a;
mod table_b;

const NAME: usize = 20;
const REC: usize = NAME + 6;

/// One of the 148 CSS named colours or a system colour (as Chromium's
/// light scheme resolves it), case-insensitive, as opaque ARGB.
/// `transparent` is transparent black.
pub fn named(name: &str) -> Option<u32> {
    let b = name.as_bytes();
    if b.is_empty() || b.len() > NAME {
        return None;
    }
    if name.eq_ignore_ascii_case("transparent") {
        return Some(0);
    }
    let mut key = [b' '; NAME];
    for (k, c) in key.iter_mut().zip(b) {
        *k = c.to_ascii_lowercase();
    }
    find(table_a::ROWS, &key).or_else(|| find(table_b::ROWS, &key))
}

/// Binary search over the fixed-width records; only the last row of a
/// table may hold fewer than three.
fn find(rows: &[&str], key: &[u8; NAME]) -> Option<u32> {
    let last = rows.last()?;
    let n = (rows.len() - 1) * 3 + last.len() / REC;
    let rec = |i: usize| rows[i / 3].as_bytes().get((i % 3) * REC..(i % 3 + 1) * REC);
    let (mut lo, mut hi) = (0, n);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let r = rec(mid)?;
        match r[..NAME].cmp(&key[..]) {
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
            Ordering::Equal => {
                let hex = core::str::from_utf8(&r[NAME..]).ok()?;
                return u32::from_str_radix(hex, 16).ok().map(|c| 0xFF00_0000 | c);
            }
        }
    }
    None
}
