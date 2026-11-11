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

use crate::pm::state::{Screen, State};

use nonos_toolkit::widgets::{Cell, Rect};

use super::super::matrix_geom;
use super::super::pane_geom::table_rect;
use super::super::screens::sec_geom;
use super::super::table_geom::{self, specs, Col, COLS_OVERVIEW};
use super::Target;

// How many rows of the active list fit the pane. Navigation clamps against this,
// so it has to come from whichever geometry module actually drew the rows.
pub fn rows_visible(screen: Screen, pane: &Rect) -> usize {
    match screen {
        Screen::Authority => matrix_geom::visible_rows(pane.h),
        Screen::Overview => table_geom::visible_rows(table_rect(pane).h),
        _ => table_geom::visible_rows(pane.h),
    }
}

// The header sorts, the body selects. Both come from the shared table's own
// hit over the rows the painter hands it, so the hairline under the header and
// the room below the last row name nothing.
/* Overview's table starts below its cards: the click comes off that offset
 * before the table sees it, as the painter added it before drawing. */
pub fn overview(state: &State, r: &Rect, x: i32, y: i32) -> Option<Target> {
    let t = table_rect(r);
    table(state, &t, &COLS_OVERVIEW, x, y - (t.y - r.y) as i32)
}

pub fn table(state: &State, r: &Rect, cols: &[Col], x: i32, y: i32) -> Option<Target> {
    let n = table_geom::shown(state.scroll, state.filtered().len(), r.h);
    match table_geom::hit(&specs(cols), (r.w, r.h), (state.scroll, n), x, y)? {
        Cell::Header(k) => cols.get(k).map(|c| Target::Sort(*c)),
        Cell::Row { row, .. } => Some(Target::Row(row)),
    }
}

// A click on the process name selects the row without naming a capability.
// The matrix has nothing to sort, so its header names nothing, and its grid
// stops above the legend strip, which can never resolve as a row.
pub fn matrix(state: &State, r: &Rect, x: i32, y: i32) -> Option<Target> {
    let total = state.filtered().len();
    match matrix_geom::hit(r.w, r.h, state.scroll, total, x, y)? {
        Cell::Row { row, col: 0 } => Some(Target::Row(row)),
        Cell::Row { row, .. } => Some(Target::Matrix(row)),
        Cell::Header(_) => None,
    }
}

pub fn finding(state: &State, r: &Rect, y: i32) -> Option<Target> {
    sec_geom::row_at(r.h, y, state.alert_scroll, state.alerts.len()).map(Target::Finding)
}
