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

//! Relative devices, the wheel in both its forms, keys passed through, and
//! the raw event layout.

use super::report::{ev, NONE, TABLET};
use crate::events::{Raw, Report, Step};

#[test]
fn a_raw_event_is_type_code_value_little_endian() {
    let raw = Raw::decode([0x03, 0, 0x01, 0, 0x34, 0x12, 0, 0]);
    assert_eq!(raw, ev(3, 1, 0x1234));
    assert_eq!(Raw::decode([1, 0, 0x10, 1, 0xFF, 0xFF, 0xFF, 0xFF]).value, -1);
}

#[test]
fn a_mouse_report_sums_motion_and_the_wheel() {
    let mut r = Report::new();
    for e in [ev(2, 0, 5), ev(2, 1, -3), ev(2, 0, 2), ev(2, 8, 1)] {
        assert_eq!(r.feed(e, &NONE), Step::Gathered);
    }
    let p = r.take();
    assert_eq!((p.rel, p.wheel, p.abs_moved), ((7, -3), 1, false));
}

#[test]
fn gear_buttons_are_wheel_steps_and_a_dropped_report_is_forgotten() {
    let mut r = Report::new();
    r.feed(ev(1, 0x151, 1), &TABLET);
    r.feed(ev(1, 0x151, 0), &TABLET);
    r.feed(ev(1, 0x150, 1), &TABLET);
    r.feed(ev(1, 0x150, 1), &TABLET);
    assert_eq!(r.take().wheel, -1);
    r.feed(ev(2, 0, 9), &NONE);
    assert_eq!(r.feed(ev(0, 3, 0), &NONE), Step::Gathered, "SYN_DROPPED");
    assert_eq!(r.take().rel, (0, 0));
    assert_eq!(r.feed(ev(1, 30, 1), &NONE), Step::Key(30, 1));
    assert_eq!(r.feed(ev(5, 0, 1), &NONE), Step::Ignored);
}
