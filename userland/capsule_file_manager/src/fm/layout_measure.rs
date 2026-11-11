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

//! The live layout numbers, stashed in state each paint, and the listing's
//! `Pane` read back from them.

use super::layout::{content_w, metrics, px};
use super::manifest::WIDTH;
use super::pane::Pane;
use super::state::{State, ViewKind};

/// Recompute the geometry for the active view from the current window height
/// and stash it in state, so scroll clamping and click hit-testing use the same
/// numbers paint drew.
pub fn measure(state: &mut State, win_h: u32) {
    let m = metrics();
    match state.view {
        ViewKind::List => {
            state.row_top = m.first_row_y;
            state.row_h = m.row_h;
            let avail = win_h.saturating_sub(m.first_row_y + m.footer_h);
            state.view_rows = (avail / m.row_h).max(1) as usize;
        }
        ViewKind::Grid => {
            state.grid_cols = (content_w(px(WIDTH)) / m.grid_cell_w).max(1);
            let avail = win_h.saturating_sub(m.grid_top + m.footer_h);
            let rows = (avail / m.grid_cell_h).max(1);
            state.view_rows = (rows * state.grid_cols) as usize;
        }
    }
}

/// The listing as the painters lay it out at the size last painted.
pub fn pane(state: &State) -> Pane {
    Pane {
        win_w: state.win_w,
        win_h: state.win_h,
        grid: state.view == ViewKind::Grid,
        len: state.entries.len(),
        scroll: state.scroll,
        row_top: state.row_top,
        row_h: state.row_h,
        view_rows: state.view_rows,
        grid_cols: state.grid_cols,
    }
}
