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

use crate::browser::css::{Computed, GridSpec};

use super::contexts::grid_line_names::{area_place, Names};
use super::contexts::grid_line_tok::axis;
use super::tree::{BoxNode, GridPlace};
use super::walk::Walk;

/* Resolve each grid item's requested placement against the container's
 * name tables while they are at hand. A container that names nothing
 * still places items by number and span: the empty table stands in. Items
 * that ask for nothing (and anonymous items) stay auto-placed. */
#[inline(never)]
pub(super) fn resolve_grid_places(
    w: &Walk,
    container_id: usize,
    style: &Computed,
    kids: &mut [BoxNode],
) {
    let empty = GridSpec::default();
    let cont = w.grids.get(container_id).and_then(|s| s.as_ref()).unwrap_or(&empty);
    let area_cols = cont.areas.iter().map(|r| r.len()).max().unwrap_or(0);
    /* An auto-repeat template's column count is known only at layout. */
    let cols = style.grid_auto.is_none().then(|| (style.grid_col_n as usize).max(area_cols));
    let rows = Some((style.grid_row_n as usize).max(cont.areas.len()));
    for kid in kids.iter_mut().filter(|k| k.dom_id != 0) {
        let Some(item) =
            w.grids.get(kid.dom_id).and_then(|s| s.as_ref()).filter(|i| i.places_item())
        else {
            continue;
        };
        kid.grid_place = match &item.area {
            Some(name) => area_place(cont, name),
            None => {
                let (col, col_span) =
                    axis(&item.col_start, &item.col_end, Names { cont, cols: true }, cols);
                let (row, row_span) =
                    axis(&item.row_start, &item.row_end, Names { cont, cols: false }, rows);
                Some(GridPlace { col, row, col_span, row_span })
            }
        };
    }
}
