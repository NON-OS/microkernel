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

//! What a device's speed fixes: EP0's first size, which bMaxPacketSize0
//! values are taken, and how bInterval converts.

use crate::contexts::{ep0_max_packet, ep0_needs_descriptor, max_packet_for_speed};

#[test]
fn ep0_starts_at_the_size_each_speed_fixes() {
    assert_eq!(max_packet_for_speed(1, false), 8, "full speed starts at 8");
    assert_eq!(max_packet_for_speed(2, false), 8, "low speed is 8");
    assert_eq!(max_packet_for_speed(3, false), 64, "high speed is 64");
    assert_eq!(max_packet_for_speed(4, false), 512);
    assert_eq!(max_packet_for_speed(5, false), 512);
    // A USB 3 port whose speed id is a custom PSIV (Gen 2x2 and the like).
    assert_eq!(max_packet_for_speed(7, true), 512);
    assert!(ep0_needs_descriptor(1, false));
    assert!(!ep0_needs_descriptor(2, false) && !ep0_needs_descriptor(3, false));
    assert!(!ep0_needs_descriptor(1, true), "nothing full speed is on a USB 3 port");
}

#[test]
fn bmaxpacketsize0_is_taken_only_where_the_speed_allows_it() {
    for b in [8u8, 16, 32, 64] {
        assert_eq!(ep0_max_packet(1, false, b), Some(b as u16));
    }
    for b in [0u8, 7, 9, 48, 128, 255] {
        assert_eq!(ep0_max_packet(1, false, b), None, "full speed {b}");
    }
    assert_eq!(ep0_max_packet(3, false, 64), Some(64));
    assert_eq!(ep0_max_packet(3, false, 8), None);
    assert_eq!(ep0_max_packet(2, false, 8), Some(8));
    assert_eq!(ep0_max_packet(4, true, 9), Some(512), "an exponent at SuperSpeed");
    assert_eq!(ep0_max_packet(4, true, 64), None);
}
