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

use alloc::vec::Vec;

use crate::browser::css::Justify;

use super::contexts::flex_main::main_offsets;
use super::contexts::track_grow::grow;
use super::contexts::track_size::{Claim, Track};

/* Size `tracks` for items' `claims` in `size` px (None: indefinite) with
 * `gap` between tracks. A span-1 item sizes a content track directly; a
 * spanning item that needs more than its content tracks give spreads the
 * rest over them. Free space then grows tracks (track_grow.rs). Returns
 * each track's (offset, size), placed by justify-content, and the length. */
pub(super) fn size_tracks(
    mut t: Vec<Track>,
    claims: &[Claim],
    size: Option<i32>,
    gap: i32,
    j: Justify,
) -> (Vec<(i32, i32)>, i32) {
    for &(c, span, mn, mx) in claims.iter().filter(|c| c.1 > 1) {
        let range = c.min(t.len())..(c + span).min(t.len());
        let content: Vec<usize> = range.clone().filter(|&i| t[i].fr == 0 && t[i].auto).collect();
        let gaps = gap.saturating_mul(range.len() as i32 - 1);
        let have = |f: fn(&Track) -> i32| {
            t[range.clone()].iter().fold(gaps, |a, x| a.saturating_add(f(x)))
        };
        let (need_min, need_max) = (mn - have(|x| x.base), mx - have(|x| x.limit));
        for &i in &content {
            t[i].base += need_min.max(0) / content.len() as i32;
            t[i].limit = (t[i].limit + need_max.max(0) / content.len() as i32).max(t[i].base);
        }
    }
    let gaps = gap.saturating_mul(t.len() as i32 - 1);
    let used = t.iter().fold(gaps, |a, x| a.saturating_add(x.base));
    let left = match size {
        Some(s) => grow(&mut t, s.saturating_sub(used), j == Justify::Start),
        None => grow(&mut t, 0, false),
    };
    let (mut at, step, _) = main_offsets(left, t.len() as i32, 0, gap, j);
    let mut out: Vec<(i32, i32)> = Vec::with_capacity(t.len());
    for x in &t {
        out.push((at, x.base));
        at = at.saturating_add(x.base).saturating_add(step);
    }
    let len = out.last().map_or(0, |&(x, w)| x.saturating_add(w));
    (out, len)
}
