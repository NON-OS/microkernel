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

//! The file manager's scrolled listing at 1x, 1.25x, 1.5x and 2x in a window
//! grown by the same scale: the hover names the entries it draws, the list's
//! first row past the header strip and the grid's first card. Its own binary,
//! so setting the scale races no other test.

use apps_proofs::fm::hover::Hover;
use apps_proofs::fm::layout::{content_x, metrics};
use apps_proofs::fm::list_cols::head_h;
use apps_proofs::fm::target::Target;
use apps_proofs::fm_check::feed;
use apps_proofs::fm_window::{dims, pane};
use nonos_app_skeleton::InputKind::PointerAbs;
use nonos_toolkit::design::scale::{scale, set_scale};

#[test]
fn a_scrolled_listing_names_the_entries_it_draws_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let ((w, h), m) = (dims(1100, 700), metrics());
        let (list, mut hv) = (pane(w, h, false, 9), Hover::new());
        let row = (content_x() + scale().px(200), m.first_row_y + head_h() + 1);
        assert!(feed(&mut hv, &list, PointerAbs, row), "q={q}");
        assert_eq!(hv.over, Some(Target::Row(9)), "q={q}");
        let grid = pane(w, h, true, 11);
        assert!(feed(&mut hv, &grid, PointerAbs, (content_x() + 1, m.grid_top + 1)), "q={q}");
        assert_eq!(hv.over, Some(Target::Cell(11 - 11 % grid.grid_cols as usize)), "q={q}");
    }
    set_scale(4);
}
