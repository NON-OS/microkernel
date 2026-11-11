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

//! Hover on the shared tables: a move inside one header asks for no repaint
//! and one to the next asks for one, and the row and the header under the
//! pointer light while a press holds its row, at 1x, 1.25x, 1.5x and 2x in a
//! window grown by the same scale. Its own binary, so setting the scale races
//! no other test.

use apps_proofs::pm::state::Screen;
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::hover::hover_to;
use apps_proofs::pm::ui::hover_look::{head, row};
use apps_proofs::pm::ui::table_geom::{row_h, row_y};
use apps_proofs::pm_window::{at, table, window};
use nonos_app_skeleton::EventOutcome::{Idle, Repaint};
use nonos_toolkit::design::scale::{scale, set_scale};

fn a_move_inside_a_header_repaints_nothing_and_one_to_the_next_repaints(q: u32) {
    for screen in [Screen::Processes, Screen::Overview] {
        let (state, (t, cols, laid)) = (window(screen, 2), table(screen));
        for (k, (x, w)) in laid.iter().enumerate().take(cols.len() - 1) {
            let mut hover = at(&state, t.x + x, t.y);
            assert!(hover == Some(Target::Sort(cols[k])), "q={q}");
            let inside = at(&state, t.x + x + w - 1, t.y + row_h() - 1);
            assert!(hover_to(&mut hover, inside) == Idle, "q={q}");
            assert!(hover_to(&mut hover, at(&state, t.x + x + w, t.y)) == Repaint, "q={q}");
            assert!(hover == Some(Target::Sort(cols[k + 1])), "q={q}");
        }
    }
}

fn the_row_and_the_header_under_the_pointer_light_and_a_press_holds_its_row(q: u32) {
    let (mut state, (t, cols, _)) = (window(Screen::Processes, 3), table(Screen::Processes));
    let s = scale();
    let (x, y) = ((t.x + s.px(5)) as i32, (t.y + row_y(2) + s.px(3)) as i32);
    state.pointer.moved(x, y);
    state.hover = at(&state, x as u32, y as u32);
    assert!(row(&state) == (Some(5), None) && !head(&state, cols[0]), "q={q}");
    state.pointer.pressed(x, y);
    assert!(row(&state) == (Some(5), Some(5)), "q={q}");
    state.pointer.released();
    state.pointer.moved(x, (t.y + s.px(2)) as i32);
    state.hover = at(&state, x as u32, t.y + s.px(2));
    assert!(row(&state) == (None, None), "q={q}");
    assert!(head(&state, cols[0]) && !head(&state, cols[1]), "q={q}");
}

/// Both hover proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn hovers_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        a_move_inside_a_header_repaints_nothing_and_one_to_the_next_repaints(q);
        the_row_and_the_header_under_the_pointer_light_and_a_press_holds_its_row(q);
    }
    set_scale(4);
}
