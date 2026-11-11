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

//! The authority matrix is the shared Table: it names a row by its name or
//! its cells, never its header or the legend under it, and a move inside one
//! row asks for no repaint while one to the next asks for one.

use nonos_app_skeleton::EventOutcome::{Idle, Repaint};

use crate::metrics::{WIN_H, WIN_W};
use crate::pm::state::Screen;
use crate::pm::ui::hit::Target;
use crate::pm::ui::hover::hover_to;
use crate::pm::ui::matrix_geom::{self, grid_h, LEGEND_H};
use crate::pm::ui::pane_geom::pane_rect;
use crate::pm::ui::table_geom::{row_h, row_y};
use crate::pm_hover_tests::{at, window};
use crate::pm_table_tests::laid;

#[test]
fn the_matrix_names_a_row_by_its_name_or_cells_and_never_its_header_or_legend() {
    let (state, r) = (window(Screen::Authority, 6), pane_rect(WIN_W, WIN_H, false));
    let laid = laid(&matrix_geom::cols(), r.w);
    assert!((r.x..r.x + r.w).all(|x| at(&state, x, r.y + 1).is_none()));
    for slot in 0..matrix_geom::shown(6, 40, r.h) {
        let (y, i) = (r.y + row_y(slot), 6 + slot);
        let mut hover = at(&state, r.x + laid[1].0, y);
        assert!(at(&state, r.x, y) == Some(Target::Row(i)) && hover == Some(Target::Matrix(i)));
        assert!(hover_to(&mut hover, at(&state, r.x + r.w - 1, y + row_h() - 1)) == Idle);
        assert!(hover_to(&mut hover, at(&state, r.x + r.w - 1, y + row_h())) == Repaint);
    }
    let legend = r.y + grid_h(r.h);
    assert!((legend..legend + LEGEND_H).all(|y| at(&state, r.x + 1, y).is_none()));
}
