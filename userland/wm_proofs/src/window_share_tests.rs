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

//! One client's share of the window manager's table: it opens at most half
//! the windows, so every other program can still open one.

use crate::window::table::{MAX_WINDOWS, PER_OWNER};
use crate::window::{Visibility, Window, WindowTable};

const A: u32 = 80;
const B: u32 = 81;

fn window(owner_pid: u32, window_id: u32) -> Window {
    Window {
        owner_pid,
        window_id,
        visibility: Visibility::Visible,
        in_use: true,
        full_screen: false,
        ..Window::default()
    }
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_every_window() {
    assert!(PER_OWNER < MAX_WINDOWS);
}

/*
 * Window ids are the client's own, so one client could open every place in
 * the table under ids of its choosing, and every later window on the machine
 * was refused while it ran.
 */
#[test]
fn a_client_stops_at_its_share_and_another_still_opens() {
    let mut t = WindowTable::new();
    for id in 0..PER_OWNER as u32 {
        assert!(t.insert(window(A, id)).is_ok());
    }
    assert_eq!(t.held_by(A), PER_OWNER);
    assert!(t.insert(window(A, 9_999)).is_err(), "past its share");
    assert!(t.insert(window(B, 1)).is_ok(), "another client still opens");
}

#[test]
fn a_closed_window_gives_its_share_back() {
    let mut t = WindowTable::new();
    for id in 0..PER_OWNER as u32 {
        assert!(t.insert(window(A, id)).is_ok());
    }
    assert!(t.remove(A, 0).is_some());
    assert_eq!(t.held_by(A), PER_OWNER - 1);
    assert!(t.insert(window(A, 9_999)).is_ok(), "the closed window's place is A's again");
}
