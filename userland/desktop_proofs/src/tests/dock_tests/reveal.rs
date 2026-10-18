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

//! The bottom edge and the brand reveal a hidden dock.

use super::rig::{DOCK_TOP, HEIGHT, STORE_PID, WINDOW};
use crate::taskbar::{
    dock_pointer, dock_work, expire_taskbar_visibility, new_taskbar_state, reveal_taskbar,
    set_full_screen, Uptime,
};

/// The bottom edge brings the hidden dock; its area keeps it; leaving hides.
#[test]
fn the_bottom_edge_reveals_and_leaving_the_dock_hides() {
    let mut t = new_taskbar_state();
    set_full_screen(&mut t, STORE_PID, WINDOW, true);
    // The shell's sync closes the dock's window (server/dock_sync.rs).
    t.window_open = false;
    assert!(!dock_pointer(&mut t, 795, HEIGHT, DOCK_TOP), "four rows from the edge only");
    assert!(dock_pointer(&mut t, 799, HEIGHT, DOCK_TOP));
    assert!(t.visible);
    assert_eq!(dock_work(&t).window, Some(true), "its window is to open again");
    assert!(!dock_pointer(&mut t, DOCK_TOP, HEIGHT, DOCK_TOP), "on the dock it stays");
    expire_taskbar_visibility(&mut t, Uptime(1_000_000));
    assert!(t.visible, "the clock does not take it from under the pointer");
    assert!(dock_pointer(&mut t, DOCK_TOP - 1, HEIGHT, DOCK_TOP), "left its area");
    assert!(!t.visible);
}

/// A press on the brand shows it for 1.8 s, or for as long as the pointer
/// then stays on it.
#[test]
fn the_brand_shows_the_hidden_dock_for_a_moment() {
    let mut t = new_taskbar_state();
    set_full_screen(&mut t, STORE_PID, WINDOW, true);
    dock_pointer(&mut t, 10, HEIGHT, DOCK_TOP);
    assert!(reveal_taskbar(&mut t, Uptime(1000)));
    dock_pointer(&mut t, 20, HEIGHT, DOCK_TOP);
    assert!(t.visible, "moving on the menubar keeps it for its moment");
    assert!(!expire_taskbar_visibility(&mut t, Uptime(2799)));
    assert!(expire_taskbar_visibility(&mut t, Uptime(2800)), "then it goes");
    assert!(!t.visible);

    reveal_taskbar(&mut t, Uptime(5000));
    dock_pointer(&mut t, 750, HEIGHT, DOCK_TOP);
    expire_taskbar_visibility(&mut t, Uptime(9000));
    assert!(t.visible, "the pointer went to it");
    dock_pointer(&mut t, 300, HEIGHT, DOCK_TOP);
    assert!(!t.visible);
}
