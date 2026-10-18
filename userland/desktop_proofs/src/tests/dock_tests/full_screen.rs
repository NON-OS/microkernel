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

//! The dock hides only for a full-screen window.

use super::rig::{index_of, DOCK_TOP, HEIGHT, STORE_PID, TERMINAL_PID, WINDOW};
use crate::taskbar::{
    dock_pointer, dock_work, expire_taskbar_visibility, forget_dead_windows, new_taskbar_state,
    reveal_taskbar, set_full_screen, track_window_opened, Uptime,
};

/// A window that is not full screen never hides the dock, whatever the
/// pointer and the clock do: the wallet's window up, the pointer passing the
/// foot of the screen, and 1.8 s later the dock used to go.
#[test]
fn the_dock_stays_shown_while_no_window_is_full_screen() {
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, STORE_PID, index_of(b"app.store"));
    reveal_taskbar(&mut t, Uptime(1000));
    for (now, y) in [(2000, 799), (2801, 400), (10_000, 100), (1_000_000, 750)] {
        dock_pointer(&mut t, y, HEIGHT, DOCK_TOP);
        expire_taskbar_visibility(&mut t, Uptime(now));
        assert!(t.visible, "still shown at {now}");
    }
}

/// A full-screen window hides it; that window restored, or its close, or
/// its process ending, brings it back. Another window's full screen keeps it
/// hidden until both are gone.
#[test]
fn a_full_screen_window_hides_the_dock_until_it_is_gone() {
    let mut t = new_taskbar_state();
    assert!(set_full_screen(&mut t, STORE_PID, WINDOW, true), "hidden");
    assert!(!t.visible);
    assert_eq!(dock_work(&t).window, Some(false), "its window is to close");
    assert!(dock_work(&t).paint, "its area is to be cleared");
    assert!(!set_full_screen(&mut t, TERMINAL_PID, 1, true), "a second one");
    assert!(!set_full_screen(&mut t, STORE_PID, WINDOW, false), "the other still covers");
    assert!(set_full_screen(&mut t, TERMINAL_PID, 1, false), "both gone");
    assert!(t.visible);

    set_full_screen(&mut t, STORE_PID, WINDOW, true);
    forget_dead_windows(&mut t, |pid| pid != STORE_PID);
    assert!(t.visible, "its process ended without a close");
}
