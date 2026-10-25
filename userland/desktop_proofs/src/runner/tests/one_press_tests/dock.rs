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

//! The dock is drawn over every window and hit there.

use super::rig::{desk_with_dock, DOCK_WINDOW_ID, EDITOR, SHELL};
use crate::scene_raise::raise_by_pid;
use crate::z_order::raise;

/// The dock is drawn in the shell's chrome band, over every window, so where
/// a window reaches down across it the dock shows, and a press there goes to
/// the dock, not to the window drawn under it. Above the dock the window
/// keeps its presses.
#[test]
fn the_dock_draws_over_a_window_and_takes_the_presses_there() {
    let mut d = desk_with_dock();
    d.assert_consistent("a window over the dock's top edge");
    assert_eq!(d.shown_at(640, 740), SHELL, "the dock is drawn over the editor's lower edge");
    d.press(640, 740);
    assert_eq!(d.shell_presses, 1, "and a press there reaches the dock");
    d.press(640, 700);
    assert_eq!(d.shell_presses, 1, "above the dock the editor keeps its presses");
    assert_eq!(d.focused(), EDITOR);
}

/// A raise of the dock's window, or of the shell's layers, changes nothing:
/// the window manager ranks the shell's popup over every window whatever its
/// z, and the compositor keeps each of the shell's layers in its own band. So
/// what is shown and what is hit stay one, and an application raised after it
/// still never covers the dock.
#[test]
fn no_raise_moves_the_dock_under_a_window() {
    let mut d = desk_with_dock();
    let _ = raise(&mut d.windows, &mut d.z, SHELL, DOCK_WINDOW_ID);
    let _ = raise_by_pid(&mut d.scene, SHELL);
    d.press(640, 300);
    assert_eq!(d.focused(), EDITOR);
    d.assert_consistent("after the shell and the editor were raised");
    assert_eq!(d.shown_at(640, 740), SHELL);
    assert_eq!(d.hit(640, 740), SHELL);
    assert_eq!(d.shown_at(640, 300), EDITOR);
    assert_eq!(d.hit(640, 300), EDITOR);
}
