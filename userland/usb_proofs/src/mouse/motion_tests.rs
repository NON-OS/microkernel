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

//! Boot mouse motion: every motion and wheel byte is a signed byte, a
//! report too short to hold the motion is nothing, and bytes past the
//! wheel are not read.

use crate::hid::mouse_report::mouse_event;

/// The event's fields as a test compares them.
pub fn fields(report: &[u8], previous: u8) -> Option<(i16, i16, i8, u8)> {
    mouse_event(report, previous).map(|e| (e.dx, e.dy, e.dz, e.buttons))
}

#[test]
fn every_motion_byte_is_sign_extended() {
    for b in 0..=u8::MAX {
        let want = i16::from(b as i8);
        let (dx, dy, dz, _) = fields(&[0, b, 0, 0], 0).unwrap_or_default();
        assert_eq!(dx, want, "x byte {b:#x}");
        let (_, dy2, _, _) = fields(&[0, 0, b, 0], 0).unwrap_or_default();
        assert_eq!(dy2, want, "y byte {b:#x}");
        let (_, _, dz2, _) = fields(&[0, 0, 0, b], 0).unwrap_or_default();
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
    assert_eq!(fields(&[0, 1, 2], 0), Some((1, 2, 0, 0)));
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
