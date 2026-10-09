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

use super::huff::Table;

/// The `len`-bit code with its bits reversed, as DEFLATE packs it.
pub fn rev(code: u32, len: u8) -> usize {
    (code.reverse_bits() >> (32 - u32::from(len))) as usize
}

/// Primary entries for the codes no longer than the primary width, in
/// canonical order. A code of length `l` fills every index whose low `l`
/// bits are its reversed bits, so the table is grown by doubling: once it
/// is `1 << l` long each code is one write, and the copies made while
/// doubling repeat the shorter codes. Unused indices end up 0.
pub fn short_codes<const N: usize>(
    t: &mut Table<N>,
    lens: &[u8],
    syms: &[u16],
    mut next: [u32; 16],
    meta: &impl Fn(usize) -> u32,
) {
    let (mut cur, p) = (1usize, &mut t.primary);
    p[0] = 0;
    for &sym in syms {
        let l = lens[usize::from(sym)];
        while cur < 1 << l {
            p.copy_within(0..cur, cur);
            cur *= 2;
        }
        p[rev(next[usize::from(l)], l)] = meta(usize::from(sym)) | u32::from(l);
        next[usize::from(l)] += 1;
    }
    while cur < N {
        p.copy_within(0..cur, cur);
        cur *= 2;
    }
}
