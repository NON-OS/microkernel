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

//! Opening an app again (state/taskbar/route.rs, expect.rs). A dock click on
//! an app marked running raises a window it has, forgets a window whose
//! process is gone, and opens the app afresh once none is left; a launch
//! whose window never comes is said.

use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::expect::EXPECT_MS;
use crate::taskbar::{
    expect_window, new_taskbar_state, raise_tracked, reach_by, track_window_closed,
    track_window_opened, windows_overdue, Reach, Uptime,
};

const WINDOW: u32 = 0x4E4F_0001;

fn index_of(service: &[u8]) -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == service).expect("app in the table")
}

/// A Calculator window whose process crashed: no close ever reached the
/// shell, so the dock still marks it running. The click used to go to that
/// pid, then to any instance the registry named, and with neither it said
/// "did not open" and spawned nothing. Now the window is forgotten and the
/// app is left with none, so the caller opens it.
#[test]
fn a_running_mark_whose_process_is_gone_opens_the_app_afresh() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 300, calc);
    assert!(t.open[calc]);
    let mut asked = Vec::new();
    let raised = raise_tracked(&mut t, calc, |pid| {
        asked.push(pid);
        Reach::Gone
    });
    assert_eq!(raised, None);
    assert_eq!(asked, [300]);
    assert!(!t.open[calc], "the running mark goes with the last window");
    assert!(t.windows.is_empty());
}

/// A minimised window brought back by a dock click: its process takes the
/// focus frame, which restores it. Nothing is spawned beside it.
#[test]
fn a_minimised_window_is_raised_not_doubled() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert!(t.open[term]);
    assert_eq!(t.windows.len(), 1);
}

/// The newest window's process is gone and an older one is up: the older
/// one is raised, and only the gone one is forgotten.
#[test]
fn a_gone_newest_window_falls_back_to_the_next_one_alive() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 100, term);
    track_window_opened(&mut t, WINDOW, 101, term);
    let raised = raise_tracked(&mut t, term, |pid| if pid == 101 { Reach::Gone } else { Reach::Taken });
    assert_eq!(raised, Some(100));
    assert_eq!(t.windows.len(), 1);
    assert_eq!(t.windows[0].pid, 100);
    assert!(t.open[term]);
}

/// A process whose inbox is full is alive, with its window: it is not
/// forgotten, and no second window is opened over it.
#[test]
fn a_busy_window_is_kept() {
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 200, files);
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Busy), Some(200));
    assert!(t.open[files]);
}

/// Another app's windows are never asked or forgotten.
#[test]
fn only_the_clicked_apps_windows_are_asked() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 200, files);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Gone), None);
    assert!(t.open[files]);
    assert_eq!(t.windows.len(), 1);
}

/// A close, then an open: the window of the second open clears the wait the
/// click started, so nothing is said.
#[test]
fn a_window_that_comes_ends_its_wait() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 300, calc);
    assert_eq!(track_window_closed(&mut t, 300, WINDOW), Some(calc));
    expect_window(&mut t, calc, Uptime(1_000));
    track_window_opened(&mut t, WINDOW, 301, calc);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS * 2)), 0);
}

/// A spawn init refused, or an instance that never opened its window: past
/// the wait the app is named once, and only once.
#[test]
fn a_window_that_never_comes_is_said_once() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    expect_window(&mut t, calc, Uptime(1_000));
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS - 1)), 0);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS)), 1u64 << calc);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS * 3)), 0);
}

/// A second click while the first waits starts the wait again, and is one
/// wait, not two.
#[test]
fn a_second_click_restarts_the_wait() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    expect_window(&mut t, calc, Uptime(1_000));
    expect_window(&mut t, calc, Uptime(9_000));
    assert_eq!(t.expecting.len(), 1);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS)), 0);
    assert_eq!(windows_overdue(&mut t, Uptime(9_000 + EXPECT_MS)), 1u64 << calc);
}

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
