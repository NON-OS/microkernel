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

//! The power-off scrub of live address spaces: every leaf frame is scrubbed
//! in place, shared ones too, but a device window holds registers and is
//! never written. The walk asks only whether a frame is a device window.

use crate::leaf_release::{leaf, scrubbed_at_end, Leaf};

#[test]
fn a_frame_of_memory_is_scrubbed_at_the_end_of_boot() {
    assert!(scrubbed_at_end(leaf(false, false)));
    assert!(scrubbed_at_end(Leaf::Kept), "a shared frame goes too: the machine stops");
}

#[test]
fn a_device_window_is_never_written() {
    assert!(!scrubbed_at_end(leaf(false, true)));
    assert!(!scrubbed_at_end(Leaf::Device));
}
