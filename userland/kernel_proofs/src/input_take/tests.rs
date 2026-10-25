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

//! Taking from a ring of keystrokes, as drain_input does under its lock.

use super::take::take;

/// A ring of 8 holding 1..=5 at slots 6, 7, 0, 1, 2: tail 6, head 3.
fn wrapped() -> ([u32; 8], usize, usize) {
    ([3, 4, 5, 0, 0, 0, 1, 2], 3, 6)
}

#[test]
fn every_event_handed_out_is_cleared_from_its_slot() {
    let (mut buf, head, mut tail) = wrapped();
    let mut out = [0u32; 8];
    assert_eq!(take(&mut buf, head, &mut tail, &mut out), 5);
    assert_eq!(&out[..5], &[1, 2, 3, 4, 5], "in order, across the wrap");
    assert_eq!(buf, [0; 8], "no keystroke left behind");
    assert_eq!(tail, head, "the ring is empty");
}

#[test]
fn a_short_buffer_takes_and_clears_only_what_it_holds() {
    let (mut buf, head, mut tail) = wrapped();
    let mut out = [0u32; 2];
    assert_eq!(take(&mut buf, head, &mut tail, &mut out), 2);
    assert_eq!(out, [1, 2]);
    assert_eq!(buf, [3, 4, 5, 0, 0, 0, 0, 0], "the rest wait in place");
    assert_eq!(tail, 0, "wrapped past the last slot");
}

#[test]
fn an_empty_ring_or_buffer_takes_nothing() {
    let (mut buf, _, _) = wrapped();
    let mut tail = 3;
    assert_eq!(take(&mut buf, 3, &mut tail, &mut [0u32; 4]), 0, "tail at head");
    let (mut buf, head, mut tail) = wrapped();
    assert_eq!(take(&mut buf, head, &mut tail, &mut []), 0, "no room");
    assert_eq!(buf, wrapped().0, "nothing cleared");
}

#[test]
fn a_tail_outside_the_ring_takes_nothing() {
    let (mut buf, _, _) = wrapped();
    let mut tail = 8;
    assert_eq!(take(&mut buf, 3, &mut tail, &mut [0u32; 4]), 0);
    assert_eq!(tail, 8);
}
