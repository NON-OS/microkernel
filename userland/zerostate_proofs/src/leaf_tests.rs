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

//! A dying address space's leaf frames: one the process held alone is
//! scrubbed before it is freed; one still shared, or a device window, is
//! neither scrubbed nor freed here.

use std::cell::RefCell;

use crate::leaf_release::{leaf, release, Leaf};

/// What `release` did, in order.
fn run(kept: bool, device: bool) -> Vec<&'static str> {
    let done = RefCell::new(Vec::new());
    release(
        leaf(kept, device),
        || done.borrow_mut().push("zero"),
        || done.borrow_mut().push("free"),
    );
    done.into_inner()
}

#[test]
fn an_own_frame_is_scrubbed_then_freed() {
    assert_eq!(leaf(false, false), Leaf::Own);
    assert_eq!(run(false, false), ["zero", "free"]);
}

#[test]
fn a_frame_another_process_holds_is_left_whole() {
    assert_eq!(leaf(true, false), Leaf::Kept);
    assert_eq!(run(true, false), Vec::<&str>::new());
}

#[test]
fn a_device_window_is_left_to_the_broker() {
    assert_eq!(leaf(false, true), Leaf::Device);
    assert_eq!(run(false, true), Vec::<&str>::new());
    assert_eq!(run(true, true), Vec::<&str>::new());
}

/* A frame scrubbed after it is freed may already be someone else's. */
#[test]
fn the_scrub_comes_before_the_free_every_time() {
    for (kept, device) in [(false, false), (true, false), (false, true), (true, true)] {
        let done = run(kept, device);
        let zero = done.iter().position(|d| *d == "zero");
        let free = done.iter().position(|d| *d == "free");
        assert_eq!(zero.is_some(), free.is_some(), "{kept} {device}");
        assert!(zero <= free, "{kept} {device}: {done:?}");
    }
}

/* Scrubbing a frame through a real buffer: every byte ends zero. */
#[test]
fn the_scrub_leaves_no_byte_of_the_frame() {
    let mut frame: Vec<u8> = (0..4096u32).map(|i| (i * 7 + 1) as u8).collect();
    let mut freed = false;
    release(leaf(false, false), || frame.fill(0), || freed = true);
    assert!(freed && frame.iter().all(|b| *b == 0));
}
