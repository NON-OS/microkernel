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
//! pointer light while a press holds its row.

use nonos_app_skeleton::EventOutcome::{Idle, Repaint};

use crate::pm::state::Screen;
use crate::pm::ui::hit::Target;
use crate::pm::ui::hover::hover_to;
use crate::pm::ui::hover_look::{head, row};
use crate::pm::ui::table_geom::{row_h, row_y};
use crate::pm_hover_tests::{at, window};
use crate::pm_table_tests::table;

#[test]
fn a_move_inside_a_header_repaints_nothing_and_one_to_the_next_repaints() {
    for screen in [Screen::Processes, Screen::Overview] {
        let (state, (t, cols, laid)) = (window(screen, 2), table(screen));
        for (k, (x, w)) in laid.iter().enumerate().take(cols.len() - 1) {
            let mut hover = at(&state, t.x + x, t.y);
            assert!(hover == Some(Target::Sort(cols[k])));
            assert!(hover_to(&mut hover, at(&state, t.x + x + w - 1, t.y + row_h() - 1)) == Idle);
            assert!(hover_to(&mut hover, at(&state, t.x + x + w, t.y)) == Repaint);
            assert!(hover == Some(Target::Sort(cols[k + 1])));
        }
    }
}

#[test]
fn the_row_and_the_header_under_the_pointer_light_and_a_press_holds_its_row() {
    let (mut state, (t, cols, _)) = (window(Screen::Processes, 3), table(Screen::Processes));
    let (x, y) = ((t.x + 5) as i32, (t.y + row_y(2) + 3) as i32);
    state.pointer.moved(x, y);
    state.hover = at(&state, x as u32, y as u32);
    assert!(row(&state) == (Some(5), None) && !head(&state, cols[0]));
    state.pointer.pressed(x, y);
    assert!(row(&state) == (Some(5), Some(5)));
    state.pointer.released();
    state.pointer.moved(x, t.y as i32 + 2);
    state.hover = at(&state, x as u32, t.y + 2);
    assert!(row(&state) == (None, None) && head(&state, cols[0]) && !head(&state, cols[1]));
}
