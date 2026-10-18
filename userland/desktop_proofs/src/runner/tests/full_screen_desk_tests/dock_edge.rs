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

//! The bottom edge shows the dock; green again brings it back.

use super::super::super::desk::DOCK_WINDOW_ID;
use super::dock_rig::{desk_full_screen, dock_area_shows, SHELL};
use super::rig::APP;

#[test]
fn touching_the_bottom_edge_shows_the_dock_over_the_window_and_leaving_hides_it() {
    let mut d = desk_full_screen();
    d.motion(640, 790);
    assert!(!d.taskbar.visible, "near the edge is not the edge");
    d.motion(100, 799);
    assert!(d.taskbar.visible, "the bottom edge, anywhere along it");
    assert_eq!(d.shown_at(640, 750), SHELL, "drawn over the window");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_some(), "its window open again");
    d.assert_consistent("dock revealed");

    d.motion(640, 730);
    assert!(d.taskbar.visible, "the pointer on the dock keeps it");
    d.motion(640, 400);
    assert!(!d.taskbar.visible, "leaving its area puts it away");
    assert!(dock_area_shows(&d, APP), "no residue");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_none());
    d.assert_consistent("dock put away");
}

#[test]
fn green_again_brings_the_dock_back() {
    let mut d = desk_full_screen();
    d.green(APP);
    assert!(d.taskbar.visible);
    assert_eq!(d.shown_at(640, 750), SHELL);
    assert_eq!(d.hit(640, 750), SHELL);
    d.assert_consistent("restored");
}
