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

//! bInterval converted by the device's speed into the Endpoint Context's
//! Interval.

use crate::contexts::interrupt_interval;

#[test]
fn binterval_is_converted_by_speed() {
    // Full and low speed: frames, to the largest power of two of 125 us
    // not above, 1 ms to 128 ms.
    assert_eq!(interrupt_interval(1, false, 1), 3);
    assert_eq!(interrupt_interval(1, false, 8), 6);
    assert_eq!(interrupt_interval(1, false, 10), 6, "10 ms is served every 8 ms");
    assert_eq!(interrupt_interval(2, false, 10), 6);
    assert_eq!(interrupt_interval(1, false, 32), 8);
    assert_eq!(interrupt_interval(1, false, 255), 10, "never past 128 ms");
    assert_eq!(interrupt_interval(1, false, 0), 3, "a zero bInterval is read as 1");
    // High speed and above: an exponent plus one, 1 to 16.
    assert_eq!(interrupt_interval(3, false, 1), 0);
    assert_eq!(interrupt_interval(3, false, 4), 3);
    assert_eq!(interrupt_interval(3, false, 16), 15);
    assert_eq!(interrupt_interval(3, false, 0), 0);
    assert_eq!(interrupt_interval(3, false, 200), 15);
    assert_eq!(interrupt_interval(4, true, 4), 3);
    for b in 0..=255u8 {
        for (speed, usb3) in [(1, false), (2, false), (3, false), (4, true)] {
            assert!(interrupt_interval(speed, usb3, b) <= 15, "reserved Interval for {b}");
        }
    }
}
