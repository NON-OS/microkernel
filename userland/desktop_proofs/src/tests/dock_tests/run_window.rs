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

//! Qwen's window, which the Linux personality opens as app.linux.run, is
//! matched to Qwen's tile when the shell queued its run
//! (state/taskbar/run_window.rs), so a click on the tile raises it, and
//! restores it when minimised, instead of asking for a second run the
//! kernel refuses.

use super::rig::{index_of, TERMINAL_PID};
use crate::taskbar::{
    expect_run_window, new_taskbar_state, raise_tracked, run_window_app, track_window_closed,
    track_window_opened, Reach,
};

const RUN_PID: u32 = 72;
const QWEN_WINDOW: u32 = 0x10;

#[test]
fn a_queued_runs_window_is_its_apps_and_a_click_raises_it() {
    let qwen = index_of(b"tool.qwen");
    let mut t = new_taskbar_state();
    expect_run_window(&mut t, qwen);
    let app = run_window_app(&mut t, RUN_PID, || Some(RUN_PID));
    assert_eq!(app, Some(qwen));
    track_window_opened(&mut t, QWEN_WINDOW, RUN_PID, qwen);
    assert!(t.open[qwen], "the dock marks Qwen running");
    let mut asked = None;
    let raised = raise_tracked(&mut t, qwen, |pid| {
        asked = Some(pid);
        Reach::Taken
    });
    assert_eq!(raised, Some(RUN_PID), "the click goes to the run's process, not a second run");
    assert_eq!(asked, Some(RUN_PID));
    assert_eq!(track_window_closed(&mut t, RUN_PID, QWEN_WINDOW), Some(qwen));
    assert!(!t.open[qwen]);
}

#[test]
fn a_window_of_another_process_leaves_the_wait() {
    let qwen = index_of(b"tool.qwen");
    let mut t = new_taskbar_state();
    expect_run_window(&mut t, qwen);
    assert_eq!(run_window_app(&mut t, TERMINAL_PID, || Some(RUN_PID)), None);
    assert_eq!(run_window_app(&mut t, RUN_PID, || None), None, "no run registered");
    assert_eq!(run_window_app(&mut t, 0, || Some(0)), None, "pid 0 is no process");
    assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), Some(qwen), "still awaited");
}

#[test]
fn only_the_first_run_window_is_taken() {
    let qwen = index_of(b"tool.qwen");
    let mut t = new_taskbar_state();
    expect_run_window(&mut t, qwen);
    assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), Some(qwen));
    assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), None, "a popup of the run");
}

#[test]
fn with_no_run_awaited_the_registry_is_not_asked() {
    let mut t = new_taskbar_state();
    let app = run_window_app(&mut t, RUN_PID, || panic!("asked with no run awaited"));
    assert_eq!(app, None);
    let past = t.open.len();
    expect_run_window(&mut t, past);
    assert_eq!(t.run_window, None, "past the table is no app");
}
