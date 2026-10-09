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

//! The dock's rule, shown unless a full-screen window is up
//! (state/taskbar/dock_rule.rs), and the shell's window tracking when a
//! window's close never reaches it.

use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::{
    dock_pointer, dock_work, expire_taskbar_visibility, forget_dead_windows, new_taskbar_state,
    reveal_taskbar, set_full_screen, track_window_closed, track_window_opened, Uptime,
    TASKBAR_NO_ACTIVE,
};
use crate::wm_notice::{decode_wm_notice, WmEvent, WmNotice};
use crate::wm_notify_encode::{encode_notify, NOTIFY_LEN};

fn index_of(service: &[u8]) -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == service).expect("in the table")
}

const STORE_PID: u32 = 60;
const TERMINAL_PID: u32 = 61;
const WINDOW: u32 = 0x5354_4F52;

/// Screen rows of the 800-row screen these tests use: the dock's area from
/// row 713, the bottom edge's band from 796.
const HEIGHT: u32 = 800;
const DOCK_TOP: u32 = 713;

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

/// The window manager's full-screen notification as its encoder writes it,
/// decoded by the shell; the opened and closed kinds read as before, and a
/// kind the shell does not know is dropped, not misread.
#[test]
fn the_shell_reads_the_window_managers_notifications() {
    let notice = |kind: u32, x: u32| {
        let mut frame = [0u8; NOTIFY_LEN];
        encode_notify(&mut frame, kind, 40, 7, x, 0);
        decode_wm_notice(&frame)
    };
    let of = |event| Some(Some(WmNotice { event, owner_pid: 40, window_id: 7 }));
    assert_eq!(notice(0, 100), of(WmEvent::Opened));
    assert_eq!(notice(1, 100), of(WmEvent::Closed));
    assert_eq!(notice(2, 1), of(WmEvent::FullScreen(true)));
    assert_eq!(notice(2, 0), of(WmEvent::FullScreen(false)));
    assert_eq!(notice(9, 0), Some(None));
    assert_eq!(decode_wm_notice(&[0u8; 28]), None, "not a notification");
}

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
