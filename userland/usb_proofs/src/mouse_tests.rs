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

//! Boot mouse reports: motion is a signed byte, a report too short to hold
//! the motion is nothing, bytes past the wheel are not read, and only the
//! five defined button bits ever become buttons.

use crate::hid::button_changes::button_changes;
use crate::hid::mouse_report::{mouse_event, MOUSE_BUTTONS};
use crate::hid::tablet_report::TABLET_BUTTONS;

/// The event's fields as a test compares them.
pub fn fields(report: &[u8], previous: u8) -> Option<(i16, i16, i8, u8, u8)> {
    mouse_event(report, previous).map(|e| (e.dx, e.dy, e.dz, e.buttons, e.flags))
}

/// Every button event a pair of button bytes makes under `mask`.
pub fn buttons(previous: u8, current: u8, mask: u8) -> Vec<(u32, bool)> {
    let mut out = Vec::new();
    button_changes(previous, current, mask, |b, down| out.push((b, down)));
    out
}

#[test]
fn every_motion_byte_is_sign_extended() {
    for b in 0..=u8::MAX {
        let want = i16::from(b as i8);
        let (dx, dy, dz, _, _) = fields(&[0, b, 0, 0], 0).unwrap_or_default();
        assert_eq!(dx, want, "x byte {b:#x}");
        let (_, dy2, _, _, _) = fields(&[0, 0, b, 0], 0).unwrap_or_default();
        assert_eq!(dy2, want, "y byte {b:#x}");
        let (_, _, dz2, _, _) = fields(&[0, 0, 0, b], 0).unwrap_or_default();
        assert_eq!(i16::from(dz2), want, "wheel byte {b:#x}");
        assert!((-128..=127).contains(&dx) && dy == 0 && dz == 0);
    }
    // The extremes, named.
    assert_eq!(fields(&[0, 0x80, 0x7f], 0).map(|f| (f.0, f.1)), Some((-128, 127)));
    assert_eq!(fields(&[0, 0xff, 0x01], 0).map(|f| (f.0, f.1)), Some((-1, 1)));
}

#[test]
fn a_report_too_short_to_hold_the_motion_is_nothing() {
    assert_eq!(fields(&[], 0), None);
    for a in 0..=u8::MAX {
        assert_eq!(fields(&[a], 0), None);
        for b in 0..=u8::MAX {
            assert_eq!(fields(&[a, b], 0), None);
            assert_eq!(fields(&[a, b], 0x1f), None);
        }
    }
}

#[test]
fn a_three_byte_report_has_no_wheel() {
    assert_eq!(fields(&[0, 1, 2], 0), Some((1, 2, 0, 0, 1)));
}

#[test]
fn bytes_past_the_wheel_are_never_read() {
    let base = [0x05u8, 0xfe, 0x03, 0xff];
    let want = fields(&base, 0);
    for extra in 0..=4usize {
        for fill in [0x00u8, 0x7f, 0x80, 0xff] {
            let mut r = base.to_vec();
            r.resize(4 + extra, fill);
            assert_eq!(fields(&r, 0), want, "{extra} extra bytes of {fill:#x}");
        }
    }
}

#[test]
fn only_the_five_button_bits_are_buttons() {
    for b in 0..=u8::MAX {
        let got = fields(&[b, 0, 0], 0);
        let held = b & 0x1f;
        if held == 0 {
            assert_eq!(got, None, "byte {b:#x} with nothing held is no event");
        } else {
            assert_eq!(got, Some((0, 0, 0, held, 0b010)), "byte {b:#x}");
        }
    }
}

#[test]
fn a_report_that_changes_nothing_is_no_event() {
    for held in 0..=MOUSE_BUTTONS {
        assert_eq!(fields(&[held, 0, 0, 0], held), None);
        // Padding bits alone are no change.
        assert_eq!(fields(&[held | 0xe0, 0, 0], held), None);
    }
}

#[test]
fn the_flags_say_what_the_event_carries() {
    assert_eq!(fields(&[0, 1, 0], 0).map(|f| f.4), Some(0b001));
    assert_eq!(fields(&[1, 0, 0], 0).map(|f| f.4), Some(0b010));
    assert_eq!(fields(&[0, 0, 0, 1], 0).map(|f| f.4), Some(0b100));
    assert_eq!(fields(&[1, 0, 1, 1], 0).map(|f| f.4), Some(0b111));
}

#[test]
fn button_events_name_only_defined_buttons_and_say_which_way_each_went() {
    for (mask, top) in [(MOUSE_BUTTONS, 5u32), (TABLET_BUTTONS, 3)] {
        for previous in 0..=u8::MAX {
            for current in 0..=u8::MAX {
                let got = buttons(previous, current, mask);
                let want: Vec<(u32, bool)> = (0..8u32)
                    .filter(|&bit| bit < top && ((previous ^ current) >> bit) & 1 == 1)
                    .map(|bit| (bit + 1, (current >> bit) & 1 == 1))
                    .collect();
                assert_eq!(got, want, "{previous:#x} to {current:#x} under {mask:#x}");
            }
        }
    }
}
