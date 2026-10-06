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

//! Page bitmaps for the DMA pools, kept free of kernel state so the host
//! proofs include it unchanged. Bit `n` set means page `n` is handed out.

pub(crate) const WORD_BITS: usize = 64;
const PAGE_SIZE: u64 = 4096;

/// Mark and return the first run of `pages` free pages among the first
/// `limit`, or `None` when no such run exists.
pub(crate) fn take_run(used: &mut [u64], limit: usize, pages: usize) -> Option<usize> {
    let mut run = 0usize;
    let mut start = 0usize;
    for idx in 0..limit {
        if used[idx / WORD_BITS] & (1u64 << (idx % WORD_BITS)) == 0 {
            if run == 0 {
                start = idx;
            }
            run += 1;
            if run == pages {
                for page in start..start + pages {
                    used[page / WORD_BITS] |= 1u64 << (page % WORD_BITS);
                }
                return Some(start);
            }
        } else {
            run = 0;
        }
    }
    None
}

/// The page index of `addr` in a pool of `pool_pages` at `base`, when the
/// whole run of `pages` lies inside it on a page boundary.
pub(crate) fn run_offset(base: u64, pool_pages: usize, addr: u64, pages: usize) -> Option<usize> {
    if pages == 0 || pool_pages == 0 || addr < base {
        return None;
    }
    let offset = match addr.checked_sub(base) {
        Some(v) if v % PAGE_SIZE == 0 => v as usize / PAGE_SIZE as usize,
        _ => return None,
    };
    if offset.checked_add(pages).is_none_or(|end| end > pool_pages) {
        return None;
    }
    Some(offset)
}

/// Clear the run of `pages` starting at page `offset`.
pub(crate) fn clear_run(used: &mut [u64], offset: usize, pages: usize) {
    for page in offset..offset + pages {
        used[page / WORD_BITS] &= !(1u64 << (page % WORD_BITS));
    }
}

/// How many pages are handed out.
pub(crate) fn count_used(used: &[u64]) -> usize {
    used.iter().map(|w| w.count_ones() as usize).sum()
}
