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

//! A window whose process ended is forgotten, its mark kept while another runs.

use super::rig::{index_of, STORE_PID, TERMINAL_PID, WINDOW};
use crate::taskbar::{
    forget_dead_windows, new_taskbar_state, track_window_closed, track_window_opened,
    TASKBAR_NO_ACTIVE,
};

/// The Marketplace closed with Esc and its close never reached the shell: the
/// menubar went on naming it and the dock on marking it. Its process ending
/// (or the shell finding it gone) now clears both.
#[test]
fn a_window_whose_process_ended_is_forgotten_with_its_name_and_mark() {
    let store = index_of(b"app.store");
    let terminal = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 1, TERMINAL_PID, terminal);
    track_window_opened(&mut t, WINDOW, STORE_PID, store);
    assert_eq!(t.active as usize, store, "the menubar names the Marketplace");

    let lost = forget_dead_windows(&mut t, |pid| pid != STORE_PID);
    assert_eq!(lost, 1 << store);
    assert!(!t.open[store], "the dock no longer marks it running");
    assert_eq!(t.active as usize, terminal, "the menubar names the window still open");
    assert!(t.open[terminal]);
    assert_eq!(forget_dead_windows(&mut t, |pid| pid != STORE_PID), 0, "once only");

    // Its close arriving late changes nothing more.
    assert_eq!(track_window_closed(&mut t, STORE_PID, WINDOW), None);

    forget_dead_windows(&mut t, |_| false);
    assert_eq!(t.active, TASKBAR_NO_ACTIVE, "no app is named once every window is gone");
    assert!(t.visible);
}

#[test]
fn an_instance_ending_keeps_the_mark_while_another_is_open() {
    let terminal = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 1, TERMINAL_PID, terminal);
    track_window_opened(&mut t, 1, 77, terminal);
    forget_dead_windows(&mut t, |pid| pid != 77);
    assert!(t.open[terminal]);
    assert_eq!(t.active as usize, terminal);
}
