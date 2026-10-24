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

//! The dock hides under a full-screen window, and presses follow.

use super::super::super::desk::DOCK_WINDOW_ID;
use super::dock_rig::{desk_full_screen, dock_area_shows, DOCK, SHELL};
use super::rig::APP;

#[test]
fn the_dock_hides_while_a_full_screen_window_shows_and_its_band_is_the_window() {
    let mut d = desk_full_screen();
    assert!(!d.taskbar.visible);
    assert!(dock_area_shows(&d, APP), "no panel, no shadow, no band: the window");
    assert!(d.chrome.iter().all(|&c| c == 0), "nothing left on the chrome");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_none(), "the dock's window is closed");
    // The commit covered the shadow as well as the panel, so the compositor
    // drew the window there: a full recomposite matches the screen.
    let area = *d.dock_commits.last().expect("committed");
    assert!(area.x <= DOCK.0 - 3 && area.y <= DOCK.1 - 3);
    assert!(area.x + area.width >= DOCK.0 + DOCK.2 + 3 && area.y + area.height == 800);
    d.assert_consistent("dock hidden over the full-screen window");
}

#[test]
fn a_press_at_the_bottom_goes_to_the_app_while_hidden_and_to_the_dock_while_shown() {
    let mut d = desk_full_screen();
    assert_eq!(d.hit(640, 750), APP);
    d.press(640, 750);
    d.release(640, 750);
    assert_eq!(d.shell_presses, 0, "the app took it");

    d.motion(640, 799);
    assert_eq!(d.hit(640, 750), SHELL);
    d.press(640, 750);
    assert_eq!(d.shell_presses, 1, "the dock took it");
}
