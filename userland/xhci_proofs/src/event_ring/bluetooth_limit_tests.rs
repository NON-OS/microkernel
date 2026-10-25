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

//! How much one interrupt read takes: the endpoint's whole payload per
//! interval, so a HID boot report stays at 8 and an HCI event endpoint
//! reads its 16 or 64; the high-bandwidth bits of wMaxPacketSize count
//! extra packets at high speed, never bytes, and are not read below it.

use super::bluetooth_tests::EVENT_PACKET;
use crate::contexts::{interrupt_payload, SPEED_FULL, SPEED_HIGH};
use crate::protocol::INTERRUPT_PAYLOAD_MAX;

#[test]
fn an_interrupt_read_is_the_endpoints_payload_and_never_past_its_bound() {
    let esit = |speed, mps| interrupt_payload(speed, false, mps, 0).map(|p| p.esit);
    assert_eq!(esit(SPEED_FULL, 8), Some(8), "a HID boot report stays 8");
    assert_eq!(esit(SPEED_FULL, EVENT_PACKET), Some(16));
    assert_eq!(esit(SPEED_FULL, 64), Some(64));
    // Bits 12:11 count extra transactions per microframe, not bytes.
    assert_eq!(esit(SPEED_FULL, 0x1800 | 16), Some(16), "not read below high speed");
    assert_eq!(esit(SPEED_HIGH, 0x0800 | 16), Some(32), "one more packet");
    for mps in 0..=u16::MAX {
        for speed in [SPEED_FULL, SPEED_HIGH] {
            if let Some(n) = esit(speed, mps) {
                assert!(n <= INTERRUPT_PAYLOAD_MAX, "{mps:#x}");
                assert!(n >= mps & 0x7FF && n != 0, "{mps:#x}");
            }
        }
    }
}
