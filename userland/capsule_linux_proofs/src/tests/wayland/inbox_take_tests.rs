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

//! The input pump passes over a frame that is not input and keeps draining:
//! a focus notice no longer stalls the input queued behind it.

#[path = "../../../../capsule_linux/src/linux/wayland/input/took.rs"]
mod took;

use took::{took, Took};

const CAP: usize = 40;

#[test]
fn an_empty_inbox_or_a_failed_receive_ends_the_pass() {
    assert_eq!(took(0, CAP), Took::Done);
    assert_eq!(took(-11, CAP), Took::Done);
}

#[test]
fn a_frame_shorter_than_input_is_passed_over_not_the_end() {
    for n in [1, 8, 28, 39] {
        assert_eq!(took(n, CAP), Took::Skip, "{n} bytes");
    }
}

#[test]
fn a_whole_frame_is_read() {
    assert_eq!(took(40, CAP), Took::Frame);
    assert_eq!(took(41, CAP), Took::Frame);
}
