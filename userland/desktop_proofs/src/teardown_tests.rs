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

//! A closed window taken down when one of the calls fails
//! (app_skeleton runner/teardown_steps.rs). The window manager hears of the
//! close whatever the compositor answered: before, a scene remove answered
//! late kept the window in the window manager's table, a target for presses
//! with nothing drawn there, and the dock kept the app marked running.

use super::teardown_steps::{take_down, Steps};

#[derive(Default)]
struct Log {
    fail_scene: bool,
    fail_release: bool,
    calls: Vec<&'static str>,
}

impl Steps for Log {
    fn scene_remove(&mut self) -> bool {
        self.calls.push("scene_remove");
        !self.fail_scene
    }
    fn unsubscribe_input(&mut self) {
        self.calls.push("unsubscribe");
    }
    fn release_surface(&mut self) -> bool {
        self.calls.push("release");
        !self.fail_release
    }
    fn unmap_backing(&mut self) -> bool {
        self.calls.push("unmap");
        true
    }
    fn wm_close(&mut self) -> bool {
        self.calls.push("wm_close");
        true
    }
}

#[test]
fn a_late_scene_remove_still_closes_the_window_in_the_window_manager() {
    let mut log = Log { fail_scene: true, ..Log::default() };
    assert!(!take_down(&mut log));
    assert_eq!(log.calls, ["scene_remove", "unsubscribe", "release", "unmap", "wm_close"]);
}

#[test]
fn the_backing_is_not_unmapped_while_the_surface_is_still_shared() {
    let mut log = Log { fail_release: true, ..Log::default() };
    assert!(!take_down(&mut log));
    assert_eq!(log.calls, ["scene_remove", "unsubscribe", "release", "wm_close"]);
}

#[test]
fn a_clean_close_takes_every_step_once() {
    let mut log = Log::default();
    assert!(take_down(&mut log));
    assert_eq!(log.calls, ["scene_remove", "unsubscribe", "release", "unmap", "wm_close"]);
}

/// An on-demand instance whose window did not open ends, so the kernel
/// frees its slot; before, it went back to idle with no window, holding the
/// slot, and once every slot was held so, each later click was handed to
/// one of them and opened nothing. A base app still waits for the next open.
#[test]
fn an_instance_with_no_window_ends_and_a_base_app_waits() {
    use super::no_window::{no_window, NoWindow};
    assert_eq!(no_window(true), NoWindow::Exit);
    assert_eq!(no_window(false), NoWindow::Idle);
}
