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

use crate::browser::css::{AutoRepeat, Computed, GridTrack};

use super::contexts::grid_occupy::MAX_COLS;

/* How many times an auto-fill or auto-fit track repeats across `w`. Each
 * repetition costs its floor plus a gap, and the last needs no gap after
 * it, so the count is (w + gap) / (floor + gap). None when the container
 * is not an auto-repeat grid, or when the floor is not a definite length,
 * which gives no width to divide by and is not a valid auto-repeat. */
pub(super) fn auto_repeat_n(style: &Computed, w: i32, items: usize) -> Option<usize> {
    let mode = style.grid_auto?;
    let gap = style.column_gap as i32;
    let floor = match style.grid_auto_min {
        GridTrack::Px(p) => p as i32,
        GridTrack::Pct(p) => (w as i64 * p as i64 / 10_000) as i32,
        _ => return None,
    }
    .max(1);
    let n = (w.saturating_add(gap) / floor.saturating_add(gap)).clamp(1, MAX_COLS as i32) as usize;
    /* auto-fit drops the tracks no item lands in, so the items that do
     * exist share the whole width rather than leaving a gap at the end. */
    Some(match mode {
        AutoRepeat::Fit => n.min(items.max(1)),
        AutoRepeat::Fill => n,
    })
}
