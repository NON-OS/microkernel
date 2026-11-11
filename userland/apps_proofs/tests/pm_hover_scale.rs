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

//! What the pointer hovers in the process monitor's sidebar, at 1x, 1.25x,
//! 1.5x and 2x in a window grown by the same scale: a move inside the box an
//! entry is drawn in asks for no repaint, the gap under it names nothing, and
//! a move to the next entry asks for one. Its own binary, so setting the
//! scale races no other test.

use apps_proofs::metrics::metrics;
use apps_proofs::pm::state::{Screen, SCREENS};
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::hover::hover_to;
use apps_proofs::pm::ui::nav_geom::{row_w, row_x, row_y};
use apps_proofs::pm_window::{at, size, window};
use nonos_app_skeleton::EventOutcome::{Idle, Repaint};
use nonos_toolkit::design::scale::set_scale;

#[test]
fn a_move_inside_a_nav_entry_repaints_nothing_and_one_to_the_next_repaints_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (state, nav_h) = (window(Screen::Overview, 0), metrics().nav_h);
        let (x0, x1) = (row_x(), row_x() + row_w(size().0) - 1);
        for i in 0..SCREENS.len() - 1 {
            let mut hover = at(&state, x0, row_y(i));
            assert!(hover == Some(Target::Nav(SCREENS[i])), "q={q}: entry {i}");
            assert!(hover_to(&mut hover, at(&state, x1, row_y(i) + nav_h - 1)) == Idle, "q={q}");
            assert!(at(&state, x0, row_y(i) + nav_h).is_none(), "q={q}: the gap under {i}");
            assert!(hover_to(&mut hover, at(&state, x0, row_y(i + 1))) == Repaint, "q={q}");
            assert!(hover == Some(Target::Nav(SCREENS[i + 1])), "q={q}: entry {}", i + 1);
        }
    }
    set_scale(4);
}
