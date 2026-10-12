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

//! How much one interrupt read may ask: one packet as the endpoint
//! descriptor's wMaxPacketSize gives it, at most 64 octets, so a HID boot
//! report stays at 8 and an HCI event endpoint reads its 16 or 64.

use super::bluetooth_tests::EVENT_PACKET;
use crate::protocol::INTERRUPT_IN_MAX;
use crate::slots::interrupt_limit;

#[test]
fn an_interrupt_read_is_one_packet_and_never_past_64() {
    assert_eq!(INTERRUPT_IN_MAX, 64);
    assert_eq!(interrupt_limit(8), 8, "a HID boot report stays 8");
    assert_eq!(interrupt_limit(EVENT_PACKET), 16);
    assert_eq!(interrupt_limit(64), 64);
    assert_eq!(interrupt_limit(1023), 64);
    // Bits 12:11 count extra transactions per microframe, not bytes.
    assert_eq!(interrupt_limit(0x1800 | 16), 16);
    for mps in 0..=u16::MAX {
        assert!(interrupt_limit(mps) as usize <= INTERRUPT_IN_MAX);
        assert!(interrupt_limit(mps) <= mps & 0x7FF);
    }
}
