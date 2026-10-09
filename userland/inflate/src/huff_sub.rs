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

use super::huff::{Table, SUB};
use super::huff_fill::rev;

/// Second-level tables for the longer codes: one per primary index they
/// share, as wide as the longest code under it needs, then their entries
/// (repeated where a code is shorter than its table is wide).
pub fn long_codes<const N: usize>(
    t: &mut Table<N>,
    lens: &[u8],
    syms: &[u16],
    next: [u32; 16],
    meta: &impl Fn(usize) -> u32,
) {
    let bits = Table::<N>::BITS as u8;
    t.sub.clear();
    if syms.is_empty() {
        return;
    }
    let (mut need, mut code) = ([0u8; N], next);
    for &sym in syms {
        let l = lens[usize::from(sym)];
        let w = &mut need[rev(code[usize::from(l)], l) & (N - 1)];
        *w = (*w).max(l - bits);
        code[usize::from(l)] += 1;
    }
    for (slot, &w) in need.iter().enumerate().filter(|(_, &w)| w != 0) {
        t.primary[slot] = SUB | (t.sub.len() as u32) << 16 | u32::from(w);
        t.sub.resize(t.sub.len() + (1 << w), 0);
    }
    let mut code = next;
    for &sym in syms {
        let l = lens[usize::from(sym)];
        let r = rev(code[usize::from(l)], l);
        code[usize::from(l)] += 1;
        let head = t.primary[r & (N - 1)];
        let mut i = r >> bits;
        while i < 1 << (head & 0xFF) {
            t.sub[(head >> 16) as usize + i] = meta(usize::from(sym)) | u32::from(l);
            i += 1 << (l - bits);
        }
    }
}
