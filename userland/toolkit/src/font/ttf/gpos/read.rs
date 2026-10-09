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

/* Big-endian reads at an offset into font data; None past the end. */

pub(super) fn u16_at(d: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*d.get(at)?, *d.get(at + 1)?]))
}

pub(super) fn u32_at(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes([*d.get(at)?, *d.get(at + 1)?, *d.get(at + 2)?, *d.get(at + 3)?]))
}

/* The offset of table `tag` in an sfnt, from its table directory. */
pub(super) fn table(d: &[u8], tag: &[u8; 4]) -> Option<usize> {
    let n = u16_at(d, 4)? as usize;
    (0..n)
        .map(|i| 12 + 16 * i)
        .find(|&r| d.get(r..r + 4) == Some(&tag[..]))
        .and_then(|r| u32_at(d, r + 8))
        .map(|o| o as usize)
}

/// The last of `n` entries, sorted by the key `key(i)` reads, whose key is
/// at or below `g`; None when every key is above it or a read fails.
pub(super) fn last_at_or_below(
    n: usize,
    g: u16,
    key: impl Fn(usize) -> Option<u16>,
) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, n);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if key(mid)? <= g {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo.checked_sub(1)
}
