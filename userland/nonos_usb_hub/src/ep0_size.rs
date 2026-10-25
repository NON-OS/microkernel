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

//! A child's endpoint 0 size from its device descriptor's bMaxPacketSize0
//! (USB 2.0 9.6.1, USB 3.2 9.6.1, as recalled, unverified): an exponent at
//! SuperSpeed, where 9 is the one size (512); 8 at low speed and 64 at high
//! speed, alone; 8, 16, 32 or 64 at full speed. Any other is no size, and
//! the child is not parked on it.

use crate::port_status::{SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER};

pub fn ep0_size(speed: u8, b_max_packet_size0: u8) -> Option<u16> {
    let b = b_max_packet_size0;
    match speed {
        s if s >= SPEED_SUPER => (b == 9).then_some(512),
        SPEED_HIGH => (b == 64).then_some(64),
        SPEED_LOW => (b == 8).then_some(8),
        SPEED_FULL => matches!(b, 8 | 16 | 32 | 64).then_some(u16::from(b)),
        _ => None,
    }
}
