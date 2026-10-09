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

use crate::browser::css::{Computed, GridTrack};

use super::super::track_widths::size_tracks;
use super::super::tree::BoxNode;
use super::grid_occupy::Area;
use super::intrinsic::contribution;
use super::track_size::{per_track, tracks_for, Claim};

/* The template column `i` of `n` explicit ones, else an implicit one. */
fn col_track(s: &Computed, i: usize, n: usize) -> GridTrack {
    match (i < n, s.grid_auto.is_some()) {
        (true, true) => s.grid_cols[0],
        (true, false) if i < s.grid_col_n as usize => s.grid_cols[i.min(s.grid_cols.len() - 1)],
        _ => s.grid_auto_cols,
    }
}

/* The (offset, width) of each of `n[1]` columns, `n[0]` of them explicit,
 * in a grid `w` wide holding `items` at `placed`. Items are measured only
 * when some column sizes by its content. */
pub(in super::super) fn col_sizes(
    s: &Computed,
    items: &[&BoxNode],
    placed: &[Area],
    w: i32,
    [n, ncols]: [usize; 2],
    depth: u32,
) -> Vec<(i32, i32)> {
    let kinds = || (0..ncols).map(|i| col_track(s, i, n));
    let content = kinds()
        .any(|k| matches!(k, GridTrack::Auto | GridTrack::MinContent | GridTrack::MaxContent));
    let claim = |(it, a): (&&BoxNode, &Area)| {
        let (mn, mx) = if content { contribution(it, depth + 1) } else { (0, 0) };
        (a.c, a.cs, mn, mx)
    };
    let claims: Vec<Claim> = items.iter().zip(placed).map(claim).collect();
    let t = tracks_for(kinds(), Some(w), &per_track(&claims, ncols));
    size_tracks(t, &claims, Some(w), s.column_gap as i32, s.justify).0
}
