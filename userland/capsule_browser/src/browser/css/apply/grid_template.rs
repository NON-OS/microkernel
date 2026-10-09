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

use crate::browser::css::auto_repeat::parse_auto_repeat;
use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::{Computed, GridTrack, MAX_GRID_COLS, MAX_GRID_ROWS};
use crate::browser::css::grid_tracks::parse_grid_tracks;
use crate::browser::css::one_track::one_track;

use super::grid_halves::{auto_flow, halves};

/* grid-template-columns: a repeat(auto-fill | auto-fit, ...) form, whose
 * count comes from the width at layout, or a fixed track list. */
pub(super) fn set_cols(c: &mut Computed, value: &str, fs: u32) {
    if let Some((mode, min, track)) = parse_auto_repeat(value, fs) {
        (c.grid_auto, c.grid_auto_min) = (Some(mode), min);
        (c.grid_cols, c.grid_col_n) = ([track; MAX_GRID_COLS], 1);
    } else if let Some((cols, n)) = parse_grid_tracks::<MAX_GRID_COLS>(value, fs) {
        (c.grid_auto, c.grid_cols, c.grid_col_n) = (None, cols, n);
    } else if value.trim() == "none" {
        (c.grid_auto, c.grid_col_n) = (None, 0);
    }
}

/* grid-template and the grid shorthand: rows / columns. The rows half may
 * interleave the area strings, each followed by its row's size (auto when
 * none). In grid, an auto-flow keyword on one side sets the flow along
 * that axis and the size after it the implicit tracks'. */
pub(super) fn apply_template(c: &mut Computed, value: &str, fs: u32) {
    let (rows, cols) = halves(value.trim());
    if let Some(size) = auto_flow(rows, fs) {
        (c.grid_flow_col, c.grid_dense, c.grid_auto_rows) = (false, rows.contains("dense"), size);
    } else if rows.contains(['"', '\'']) {
        let (mut out, mut n) = ([GridTrack::Auto; MAX_GRID_ROWS], 0usize);
        for t in words(rows).filter(|t| !t.starts_with('[')) {
            match (t.starts_with(['"', '\'']), one_track(t, fs)) {
                (true, _) => n = (n + 1).min(MAX_GRID_ROWS),
                (false, Some(size)) if n > 0 => out[n - 1] = size,
                _ => {}
            }
        }
        (c.grid_rows, c.grid_row_n) = (out, n as u8);
    } else if let Some((r, n)) = parse_grid_tracks::<MAX_GRID_ROWS>(rows, fs) {
        (c.grid_rows, c.grid_row_n) = (r, n);
    }
    let Some(cols) = cols else { return };
    match auto_flow(cols, fs) {
        Some(size) => {
            (c.grid_flow_col, c.grid_dense, c.grid_auto_cols) = (true, cols.contains("dense"), size)
        }
        None => set_cols(c, cols, fs),
    }
}
