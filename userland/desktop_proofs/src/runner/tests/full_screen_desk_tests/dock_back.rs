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

//! What brings the dock back, and what keeps full screen.

use super::dock_rig::{desk_full_screen, OTHER, SHELL};
use super::rig::APP;
use crate::window::full_screen::covers_dock_at;

#[test]
fn closing_or_minimising_the_full_screen_window_brings_the_dock_back() {
    let mut d = desk_full_screen();
    d.minimize(APP);
    assert!(d.taskbar.visible, "minimised");
    assert_eq!(d.hit(640, 750), SHELL);
    d.assert_consistent("minimised");
    d.restore_window(APP);
    assert!(!d.taskbar.visible, "brought back full screen from the dock");
    d.assert_consistent("restored from the dock");

    d.close(APP);
    assert!(d.taskbar.visible, "closed");
    assert_eq!(d.shown_at(640, 750), SHELL);
    d.assert_consistent("closed");

    // A process that ends without its close: the shell forgets it once a
    // second (forget_dead_windows), and the dock comes back.
    let mut d = desk_full_screen();
    crate::taskbar::forget_dead_windows(&mut d.taskbar, |pid| pid != APP);
    d.shell_sync();
    assert!(d.taskbar.visible, "its process ended");
}

#[test]
fn another_window_opening_or_closing_keeps_full_screen() {
    let mut d = desk_full_screen();
    d.open(OTHER, (300, 200, 400, 300), 0);
    d.shell_sync();
    assert!(!d.taskbar.visible, "another window opened");
    assert!(covers_dock_at(&d.windows, APP, 0x5749_4E00 | APP));
    d.minimize(OTHER);
    assert!(!d.taskbar.visible, "another window minimised");
    d.restore_window(OTHER);
    d.close(OTHER);
    assert!(!d.taskbar.visible, "another window closed");
    assert_eq!(d.hit(640, 750), APP);
    d.assert_consistent("the other window gone");
}
