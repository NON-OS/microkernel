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

//! The file manager's Browse listing at 1x, 1.25x, 1.5x and 2x in a window
//! grown by the same scale: its sizes are the 1x ones read through the scale,
//! it fits beside the info panel and above the footer, and every sort column,
//! check box, row and grid card is named where it is drawn, scrolled or not,
//! with none overlapping. Its own binary, so setting the scale races no other
//! test.

use apps_proofs::fm::layout::{content_w, content_x, info_x, metrics};
use apps_proofs::fm::list_cols::{check_w, cols, head_h};
use apps_proofs::fm::target::Target;
use apps_proofs::fm_check::{at, named_apart};
use apps_proofs::fm_window::{dims, drawn, pane};
use nonos_toolkit::design::scale::{scale, set_scale};

#[test]
fn every_listing_control_is_named_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (m, s) = (metrics(), scale());
        assert_eq!((m.row_h, m.first_row_y, head_h()), (s.px(44), s.px(84), s.px(30)), "q={q}");
        assert_eq!((m.grid_cell_w, m.grid_cell_h, m.grid_top), (s.px(124), s.px(112), s.px(90)));
        for (w, h) in [dims(1100, 700), dims(1280, 560), dims(1600, 1000)] {
            assert!(content_x() + content_w(w) <= info_x(w) && info_x(w) + m.info_w <= w, "q={q}");
            let c = cols(content_x(), content_w(w));
            assert!(c.check_x + check_w() <= c.tile_x, "q={q}: the check box fits its gutter");
            for (grid, scroll) in [(false, 0), (false, 7), (true, 0), (true, 13)] {
                let p = pane(w, h, grid, scroll);
                let cs = drawn(&p);
                assert!(cs.len() > 4, "q={q} {w}x{h} grid {grid}");
                let foot = h - m.footer_h;
                let rows = cs.iter().filter(|(t, _)| matches!(t, Target::Row(_) | Target::Cell(_)));
                assert!(rows.clone().all(|(_, r)| r.y + r.h <= foot), "q={q}: above the footer");
                let what = format!("q={q} {w}x{h} at {scroll}");
                named_apart(&cs, |x, y| at(&p, x as i32, y as i32), &what);
            }
        }
    }
    set_scale(4);
}
