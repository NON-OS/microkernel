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

//! Each close takes every step, once and in an order that frees nothing early.

use super::super::super::teardown_steps::take_down;
use super::log::Log;

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
    use super::super::super::no_window::{no_window, NoWindow};
    assert_eq!(no_window(true), NoWindow::Exit);
    assert_eq!(no_window(false), NoWindow::Idle);
}
