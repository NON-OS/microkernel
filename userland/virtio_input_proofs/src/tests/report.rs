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

//! Linux events into reports: positions scaled from the device's range,
//! motion and the wheel summed, buttons queued, the gathered report handed
//! over on SYN_REPORT and thrown away on SYN_DROPPED.

use crate::events::{Raw, Report, Step};
use crate::init::{Axes, Axis};

pub const TABLET: Axes =
    Axes { x: Some(Axis { min: 0, max: 0x7FFF }), y: Some(Axis { min: 0, max: 0x7FFF }) };
const WIDE: Axes =
    Axes { x: Some(Axis { min: 0, max: 0xFFFF }), y: Some(Axis { min: -100, max: 100 }) };
pub const NONE: Axes = Axes { x: None, y: None };

pub fn ev(kind: u16, code: u16, value: i32) -> Raw {
    Raw { kind, code, value }
}

#[test]
fn a_tablet_report_places_the_pointer_then_clicks_there() {
    let mut r = Report::new();
    assert_eq!(r.feed(ev(3, 0, 1000), &TABLET), Step::Gathered);
    assert_eq!(r.feed(ev(3, 1, 2000), &TABLET), Step::Gathered);
    assert_eq!(r.feed(ev(1, 0x110, 1), &TABLET), Step::Gathered);
    assert_eq!(r.feed(ev(0, 0, 0), &TABLET), Step::Flush);
    let p = r.take();
    assert_eq!((p.abs, p.abs_moved, p.n_buttons, p.buttons[0]), ((1000, 2000), true, 1, (1, true)));
    // A report that moves one axis leaves the other where it was.
    r.feed(ev(3, 1, 300), &TABLET);
    let p = r.take();
    assert_eq!((p.abs, p.n_buttons), ((1000, 300), 0));
}

#[test]
fn positions_are_scaled_from_the_axis_range_the_device_gives() {
    let mut r = Report::new();
    r.feed(ev(3, 0, 0x8000), &WIDE);
    r.feed(ev(3, 1, 50), &WIDE);
    // Each axis by its own range: x halfway across 0..0xFFFF, y three
    // quarters across -100..100.
    assert_eq!(r.take().abs, (0x3FFF, 0x5FFF));
    // No range given: the device is taken to report in the absolute range.
    r.feed(ev(3, 0, 500), &NONE);
    assert_eq!(r.take().abs.0, 500);
}
