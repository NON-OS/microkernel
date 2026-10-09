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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::{Computed, GridTrack, MAX_GRID_ROWS};
use crate::browser::css::grid_tracks::parse_grid_tracks;
use crate::browser::css::one_track::one_track;
use crate::browser::css::parse_px::parse_px;

/* Grid templates, implicit track sizes, the auto-placement flow, and the
 * row and column gaps that grid and flex containers share. */
pub(super) fn apply_grid(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "grid-template-columns" => super::grid_template::set_cols(c, value, fs),
        "grid-template" | "grid" => super::grid_template::apply_template(c, value, fs),
        "grid-template-rows" => {
            if let Some((rows, n)) = parse_grid_tracks::<MAX_GRID_ROWS>(value, fs) {
                (c.grid_rows, c.grid_row_n) = (rows, n);
            } else if value.trim() == "none" {
                (c.grid_rows, c.grid_row_n) = ([GridTrack::Auto; MAX_GRID_ROWS], 0);
            }
        }
        "grid-auto-rows" => c.grid_auto_rows = one_track(value, fs).unwrap_or(c.grid_auto_rows),
        "grid-auto-columns" => c.grid_auto_cols = one_track(value, fs).unwrap_or(c.grid_auto_cols),
        "grid-auto-flow" => {
            let v = value.trim();
            c.grid_flow_col = v.contains("column");
            c.grid_dense = v.contains("dense");
        }
        "gap" | "grid-gap" => {
            /* One value sets both; two give the row gap, then the column. */
            let mut it = words(value).map(|w| gap(w, fs));
            match (it.next(), it.next(), it.next()) {
                (Some(Some(r)), None, None) => (c.row_gap, c.column_gap) = (r, r),
                (Some(Some(r)), Some(Some(k)), None) => (c.row_gap, c.column_gap) = (r, k),
                _ => {}
            }
        }
        "row-gap" | "grid-row-gap" => c.row_gap = gap(value, fs).unwrap_or(c.row_gap),
        "column-gap" | "grid-column-gap" => c.column_gap = gap(value, fs).unwrap_or(c.column_gap),
        _ => return false,
    }
    true
}

/* A gap length; normal is no gap. A percentage has no px value here. */
fn gap(v: &str, fs: u32) -> Option<u32> {
    (v.trim() == "normal").then_some(0).or_else(|| parse_px(v, fs))
}
