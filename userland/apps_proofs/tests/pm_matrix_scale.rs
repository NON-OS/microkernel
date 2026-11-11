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
//! row asks for no repaint while one to the next asks for one, at 1x, 1.25x,
//! 1.5x and 2x in a window grown by the same scale. Its own binary, so setting
//! the scale races no other test.

use apps_proofs::pm::state::Screen;
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::hover::hover_to;
use apps_proofs::pm::ui::matrix_geom::{self, grid_h, LEGEND_H};
use apps_proofs::pm::ui::pane_geom::pane_rect;
use apps_proofs::pm::ui::table_geom::{row_h, row_y};
use apps_proofs::pm_window::{at, laid, size, window};
use nonos_app_skeleton::EventOutcome::{Idle, Repaint};
use nonos_toolkit::design::scale::{scale, set_scale};

#[test]
fn the_matrix_names_a_row_by_its_name_or_cells_and_never_its_header_or_legend_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let ((w, h), state) = (size(), window(Screen::Authority, 6));
        let r = pane_rect(w, h, false);
        let laid = laid(&matrix_geom::cols(), r.w);
        assert!((r.x..r.x + r.w).all(|x| at(&state, x, r.y + 1).is_none()), "q={q}: header");
        for slot in 0..matrix_geom::shown(6, 40, r.h) {
            let (y, i) = (r.y + row_y(slot), 6 + slot);
            let mut hover = at(&state, r.x + laid[1].0, y);
            assert!(at(&state, r.x, y) == Some(Target::Row(i)), "q={q}: name {i}");
            assert!(hover == Some(Target::Matrix(i)), "q={q}: cells {i}");
            let inside = at(&state, r.x + r.w - 1, y + row_h() - 1);
            assert!(hover_to(&mut hover, inside) == Idle, "q={q}");
            let next = at(&state, r.x + r.w - 1, y + row_h());
            assert!(hover_to(&mut hover, next) == Repaint, "q={q}");
        }
        let legend = r.y + grid_h(r.h);
        assert_eq!(r.y + r.h - legend, scale().px(LEGEND_H), "q={q}: the legend's band");
        let named =
            (legend..legend + scale().px(LEGEND_H)).any(|y| at(&state, r.x + 1, y).is_some());
        assert!(!named, "q={q}: the legend names a row");
    }
    set_scale(4);
}
