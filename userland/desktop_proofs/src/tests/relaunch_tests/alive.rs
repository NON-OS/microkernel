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

//! What counts as a process still alive.

use super::rig::{index_of, WINDOW};
use crate::taskbar::expect::EXPECT_MS;
use crate::taskbar::{
    expect_window, new_taskbar_state, raise_tracked, reach_by, track_window_opened,
    windows_overdue, Reach, Uptime,
};

/// An app that has just ended (red close, quit or crash) is a zombie until
/// its tables are freed. The kernel still takes a send into its inbox then,
/// so the send alone read it as Taken: the dock click raised nothing and
/// opened nothing. It is Gone, without a frame sent, and the click opens
/// the app afresh.
#[test]
fn a_zombie_whose_inbox_still_takes_a_frame_is_gone() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 300, calc);
    let mut sent = Vec::new();
    let mut send = |p| {
        sent.push(p);
        0
    };
    let raised = raise_tracked(&mut t, calc, |pid| reach_by(pid, |_| false, &mut send));
    assert_eq!(raised, None, "the app is opened afresh");
    assert!(sent.is_empty(), "nothing is sent into a dead process's inbox");
    assert!(!t.open[calc]);
    assert!(t.windows.is_empty());
}

/// A live process: the kernel's answer to the send decides. A full inbox is
/// Busy and its window kept; a refused send is Gone.
#[test]
fn a_live_process_is_read_by_the_send() {
    assert_eq!(reach_by(7, |_| true, |_| 0), Reach::Taken);
    assert_eq!(reach_by(7, |_| true, |_| -16), Reach::Busy);
    assert_eq!(reach_by(7, |_| true, |_| -2), Reach::Gone);
    assert_eq!(reach_by(7, |_| true, |_| -1), Reach::Gone);
}

/// The N4120: an attested spawn of a large app (Browser: a 115 KB proof to
/// check, its ELF to load, its JavaScript engine to build) can take well
/// past 15 s while init is busy. A window that comes at 25 s is not said
/// to have failed before it appears.
#[test]
fn a_slow_window_is_not_said_to_have_failed() {
    let browser = index_of(b"app.browser");
    let mut t = new_taskbar_state();
    expect_window(&mut t, browser, Uptime(1_000));
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + 25_000)), 0);
    track_window_opened(&mut t, WINDOW, 400, browser);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS * 2)), 0);
}
