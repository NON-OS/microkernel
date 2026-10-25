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

//! Which surface the pointer and keyboard are told they are on: the shown
//! window's, told again when the window comes to show another, and never
//! a surface the client has destroyed.

use crate::wayland_focus_step::{step, Step};

fn all_alive(_: u32) -> bool {
    true
}

#[test]
fn nothing_shown_drops_the_event() {
    assert_eq!(step(None, None, all_alive), Step::Drop);
    assert_eq!(step(Some(3), None, all_alive), Step::Drop);
}

#[test]
fn the_first_event_enters_the_shown_surface_not_the_first_made() {
    assert_eq!(step(None, Some(9), all_alive), Step::Move { leave: None, on: 9 });
    assert_eq!(step(Some(9), Some(9), all_alive), Step::Stay);
}

#[test]
fn a_new_window_leaves_the_old_surface_while_the_client_holds_it() {
    assert_eq!(step(Some(9), Some(12), all_alive), Step::Move { leave: Some(9), on: 12 });
    assert_eq!(step(Some(9), Some(12), |id| id != 9), Step::Move { leave: None, on: 12 });
}
