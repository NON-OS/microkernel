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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::Table;

use crate::pm::state::State;

use super::super::chrome::Rect;
use super::super::matrix_geom as geom;
use super::super::table_geom::row_y;
use super::super::{badge, hover_look};
use super::{auth_cells, auth_legend};

// The screen the hex `caps` dump could never be: one row per process, one column
// per sensitive authority, so an unexpected grant is a shape rather than a number
// nobody decodes. The shared table draws the header and the names; each grant is
// marked over its cell, in its class colour, which a table of text cannot draw.
// Every rect comes out of matrix_geom, which is also what the hit test reads, so
// a click cannot land on a cell other than the drawn one.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let rows = state.filtered();
    let first = state.scroll.min(rows.len());
    let shown = &rows[first..first + geom::shown(first, rows.len(), r.h)];
    let (cols, (hovered, pressed)) = (geom::cols(), hover_look::row(state));
    let (grid, head) = (Rect { h: geom::grid_h(r.h), ..*r }, Table::new(&cols));
    let ink = head.head_rect(grid, 0).map(Table::text_rect).unwrap_or_default();
    let names: Vec<String> =
        shown.iter().map(|row| badge::name(row.name(), ink.w, state.is_protected(row))).collect();
    let lines: Vec<[&str; 1]> = names.iter().map(|n| [n.as_str()]).collect();
    let body: Vec<&[&str]> = lines.iter().map(|l| &l[..]).collect();
    let pid = state.selected_pid;
    let selected = rows.iter().position(|row| pid != 0 && row.pid == pid);
    let table = Table::new(&cols).rows(&body).first(state.scroll).selected(selected);
    table.hovered(hovered).pressed(pressed).draw(fb, grid);
    let centres = auth_cells::centres(&head, grid);
    for (k, (row, name)) in shown.iter().zip(&names).enumerate() {
        let (y, protected) = (r.y + row_y(k), state.is_protected(row));
        if protected {
            badge::shield(fb, ink.x, y, name);
        }
        auth_cells::paint(fb, y, &centres, row.caps, protected);
    }
    auth_legend::paint(fb, r);
}
