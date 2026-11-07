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

//! The fetch plan asks for the lowest chunk not held and not in flight, and
//! never more than its width at once.

use nonos_app_image::next;

#[test]
fn one_at_a_time_in_order() {
    let mut held = vec![false; 4];
    let mut order = Vec::new();
    while let Some(i) = next(&held, &[], 1) {
        order.push(i);
        held[i] = true;
    }
    assert_eq!(order, [0, 1, 2, 3]);
    assert_eq!(next(&[false, false], &[0], 1), None, "width 1 is full");
}

#[test]
fn held_chunks_are_never_asked_again() {
    let held = [true, false, true, false];
    assert_eq!(next(&held, &[], 4), Some(1));
    assert_eq!(next(&held, &[1], 4), Some(3));
    assert_eq!(next(&held, &[1, 3], 4), None);
    assert_eq!(next(&[true; 3], &[], 4), None, "a cache hit asks for nothing");
}

#[test]
fn width_bounds_what_is_in_flight() {
    let held = [false; 8];
    assert_eq!(next(&held, &[0, 1, 2], 4), Some(3));
    assert_eq!(next(&held, &[0, 1, 2, 3], 4), None);
    assert_eq!(next(&held, &[], 0), None);
}
