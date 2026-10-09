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

//! A guest's window is taken down when what made it ends
//! (wayland/window_life.rs). It never was: the compositor kept drawing it
//! and the window manager kept it, a ghost a press still reached.

use crate::window_life::{ends_connection, take_down, takes_down, End, Shown, Steps};

const SHOWN: Option<Shown> = Some(Shown { surface: 8, toplevel: Some(10) });

#[test]
fn the_toplevel_or_surface_of_the_window_takes_it_down_and_no_other() {
    assert!(takes_down(End::Toplevel(10), SHOWN));
    assert!(takes_down(End::Surface(8), SHOWN));
    assert!(!takes_down(End::Toplevel(11), SHOWN));
    assert!(!takes_down(End::Surface(9), SHOWN));
    assert!(!takes_down(End::Toplevel(10), None), "nothing shown");
}

#[test]
fn the_last_display_descriptor_closes_the_connection() {
    // stdin, stdout, stderr, then the display socket at 3.
    assert!(ends_connection(&[false, false, false, true], 3));
    assert!(!ends_connection(&[false, false, false, true, true], 3), "a dup keeps it open");
    assert!(!ends_connection(&[false, false, false, true], 2), "not the display");
    assert!(!ends_connection(&[false, false, false, true], 7), "no such descriptor");
}

#[derive(Default)]
struct Record {
    refuse_remove: bool,
    refuse_release: bool,
    calls: Vec<&'static str>,
}

impl Steps for Record {
    fn scene_remove(&mut self) -> bool {
        self.calls.push("scene_remove");
        !self.refuse_remove
    }
    fn release_surface(&mut self) -> bool {
        self.calls.push("release_surface");
        !self.refuse_release
    }
    fn free_pixels(&mut self) {
        self.calls.push("free_pixels");
    }
    fn keep_pixels(&mut self) {
        self.calls.push("keep_pixels");
    }
    fn wm_close(&mut self) -> bool {
        self.calls.push("wm_close");
        true
    }
}

#[test]
fn every_step_is_taken_and_the_pixels_go_after_their_surface() {
    let mut r = Record::default();
    assert!(take_down(&mut r));
    assert_eq!(r.calls, ["scene_remove", "release_surface", "free_pixels", "wm_close"]);
}

#[test]
fn a_refused_remove_still_closes_the_window_so_the_dock_comes_back() {
    let mut r = Record { refuse_remove: true, ..Record::default() };
    assert!(!take_down(&mut r));
    assert!(r.calls.contains(&"wm_close"));
    assert!(r.calls.contains(&"free_pixels"));
}

#[test]
fn pixels_the_kernel_may_still_map_are_kept() {
    let mut r = Record { refuse_release: true, ..Record::default() };
    assert!(!take_down(&mut r));
    assert_eq!(r.calls, ["scene_remove", "release_surface", "keep_pixels", "wm_close"]);
}
