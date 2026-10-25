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

//! When the shell sends a kept value: once per change in the store, all of
//! them again for a new compositor, never one out of range, and never to
//! undo a live change the store did not take.

use super::shell_keep::display_keep::{body, valid};
use super::shell_keep::display_told::Replay;

#[test]
fn a_kept_value_is_sent_once_per_change() {
    let mut r = Replay::default();
    r.compositor(9);
    assert_eq!(r.due(1, Some(8)), Some(8), "first read at start");
    r.told(1, 8);
    assert_eq!(r.due(1, Some(8)), None, "the same again");
    assert_eq!(r.due(1, Some(5)), Some(5), "Settings stored a new one");
    assert_eq!(r.due(1, None), None, "the store has none");
}

#[test]
fn a_new_compositor_is_told_everything_again() {
    let mut r = Replay::default();
    r.compositor(9);
    r.told(0, 1);
    r.compositor(9);
    assert_eq!(r.due(0, Some(1)), None, "the same compositor");
    r.compositor(12);
    assert_eq!(r.due(0, Some(1)), Some(1), "a restarted compositor");
}

#[test]
fn a_value_out_of_range_is_never_sent() {
    let r = Replay::default();
    for i in 0..6 {
        for v in 0..=u8::MAX {
            let due = r.due(i, Some(v));
            assert_eq!(due.is_some(), valid(i, v), "field {i} = {v}");
            assert_eq!(body(i, v).is_some(), valid(i, v), "field {i} = {v}");
        }
    }
}
