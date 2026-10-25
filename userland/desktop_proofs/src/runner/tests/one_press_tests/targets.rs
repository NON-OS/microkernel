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

//! A press on the tabs or the desktop moves no window.

use super::rig::{desk, EDITOR, TERMINAL};

#[test]
fn a_press_on_the_tabs_goes_to_the_tabs_and_moves_nothing() {
    let mut d = desk();
    d.press(500, 85);
    d.motion(300, 120);
    d.release(300, 120);
    let t = d.app(TERMINAL);
    assert_eq!((t.x, t.y), (60, 60));
    assert_eq!(t.accessory_events, 3, "the press, the motion off it and the release");
    assert_eq!(d.focused(), TERMINAL, "a press on the tabs still focuses the window");
}

#[test]
fn a_press_on_the_desktop_reaches_the_shell_and_moves_no_window() {
    let mut d = desk();
    d.press(20, 700);
    d.motion(40, 690);
    d.release(40, 690);
    assert_eq!(d.shell_presses, 1);
    assert_eq!(d.focused(), EDITOR);
    assert_eq!(d.app(TERMINAL).moves + d.app(EDITOR).moves, 0);
}
