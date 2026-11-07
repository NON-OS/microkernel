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

//! The size a toplevel is offered first: a window of its own the size a
//! guest window opens at, within its limits and the display; a dialog its
//! own.

use crate::wayland_toplevel_size::{offered, OFFER};

const NONE: ((u32, u32), (u32, u32)) = ((0, 0), (0, 0));

#[test]
fn a_window_of_its_own_is_offered_the_usual_size() {
    assert_eq!(offered(false, NONE, (1278, 717)), OFFER);
    assert_eq!(offered(false, NONE, (0, 0)), OFFER, "bounds not known");
}

#[test]
fn a_dialog_takes_its_own_size() {
    assert_eq!(offered(true, ((300, 200), (0, 0)), (1278, 717)), (0, 0));
}

#[test]
fn the_display_and_the_most_size_bound_it_and_the_least_size_wins() {
    assert_eq!(offered(false, NONE, (640, 480)), (640, 480));
    assert_eq!(offered(false, ((0, 0), (500, 0)), (1278, 717)), (500, 600));
    // A window that cannot be smaller than the display is not told to be.
    assert_eq!(offered(false, ((900, 700), (0, 0)), (640, 480)), (900, 700));
    // LibreOffice's splash: least and most both 648 by 200.
    assert_eq!(offered(false, ((648, 200), (648, 200)), (1278, 717)), (648, 200));
}
