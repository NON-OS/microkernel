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

//! The requests byte for byte, against the values worked out by hand from
//! the layout: source 0x51, length 0x80 and the count, the message, and an
//! XOR checksum over the display's write address 0x6E and every byte.

use crate::ddc::frame::{get_request, set_request, DDC_CI_ADDR, DISPLAY};

#[test]
fn a_get_vcp_for_brightness_is_the_five_bytes_by_hand() {
    // 0x6E ^ 0x51 ^ 0x82 ^ 0x01 ^ 0x10 = 0xAC.
    assert_eq!(get_request(0x10), [0x51, 0x82, 0x01, 0x10, 0xAC]);
    assert_eq!((DDC_CI_ADDR, DISPLAY), (0x37, 0x6E));
}

#[test]
fn a_set_vcp_carries_the_value_high_byte_first() {
    // 0x6E ^ 0x51 ^ 0x84 ^ 0x03 ^ 0x12 ^ 0x01 ^ 0x2C = 0x87.
    assert_eq!(set_request(0x12, 300), [0x51, 0x84, 0x03, 0x12, 0x01, 0x2C, 0x87]);
}

#[test]
fn every_request_sums_to_zero_with_the_display_address() {
    for code in 0..=255u8 {
        let g = get_request(code);
        assert_eq!(g.iter().fold(0x6E, |a, b| a ^ b), 0);
        for v in [0u16, 1, 0x00FF, 0x0100, 0xFFFF] {
            assert_eq!(set_request(code, v).iter().fold(0x6E, |a, b| a ^ b), 0);
        }
    }
}
