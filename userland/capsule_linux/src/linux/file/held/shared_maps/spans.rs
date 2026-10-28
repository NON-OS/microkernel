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

//! Which bytes of one map a call reaches: a munmap's hole, a write of its
//! file, a cut or a growth of it, and a write-back. Pure.

use super::entry::{clip, overlap, Entry};
use crate::linux::guest::{page_down, page_up, PAGE};

/// What of `e` is left around the hole `[at, at + span)`: below, above.
pub fn outside(e: &Entry, at: u64, span: u64) -> [Option<Entry>; 2] {
    let ((lo, hi), cut) = (e.place(), (at, at.saturating_add(span)));
    if overlap((lo, hi), cut).is_none() {
        return [Some(e.clone()), None];
    }
    let below = (lo < cut.0).then(|| Entry { span: cut.0 - lo, ..e.clone() });
    let off = e.off.saturating_add(cut.1.saturating_sub(lo));
    let above = (cut.1 < hi).then(|| Entry { at: cut.1, span: hi - cut.1, off, ..e.clone() });
    [below, above]
}

/// The bytes of `e`'s file that `[at, at + span)` of its process shows.
pub fn going(e: &Entry, at: u64, span: u64) -> Option<(u64, u64)> {
    let (lo, hi) = overlap(e.place(), (at, at.saturating_add(span)))?;
    Some((e.off.saturating_add(lo - e.at), e.off.saturating_add(hi - e.at)))
}

/// Where the `n` bytes written at `at` of `e`'s file land in its process,
/// and which of them: the address, then the first and the end.
pub fn echo(e: &Entry, at: u64, n: u64) -> Option<(u64, usize, usize)> {
    let (lo, hi) = overlap(e.file(), (at, at.saturating_add(n)))?;
    Some((e.addr(lo), (lo - at) as usize, (hi - at) as usize))
}

/// What of `e` reads zero once its file went from `was` to `len` bytes,
/// shorter or longer: the address and the length.
pub fn zeroed(e: &Entry, was: u64, len: u64) -> Option<(u64, u64)> {
    let (lo, hi) = overlap(e.file(), (was.min(len), was.max(len)))?;
    Some((e.addr(lo), hi - lo))
}

/// The bytes of the file a write-back of `e` for `want` covers: the whole
/// pages of `e` that `want` touches, cut at the file's end, since a map
/// never makes its file longer.
pub fn back_range(e: &Entry, want: (u64, u64), end: u64) -> Option<(u64, u64)> {
    clip(overlap(e.file(), (page_down(want.0), page_up(want.1)))?, end)
}

/// The pages of `map` whose bytes differ from `file`'s, as ranges of both.
/// Both start on a page; past the shorter there is nothing to compare.
pub fn differing(map: &[u8], file: &[u8]) -> alloc::vec::Vec<(usize, usize)> {
    let (n, page) = (map.len().min(file.len()), PAGE as usize);
    let pages = (0..n).step_by(page).map(|lo| (lo, (lo + page).min(n)));
    pages.filter(|&(lo, hi)| map[lo..hi] != file[lo..hi]).collect()
}
