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

//! The tables that tell an I2C function from its UART and SPI neighbours,
//! and name it as the firmware does.

use crate::constants::{controller_index, device_info};
use crate::setup::CLOCK_HZ;

#[test]
fn only_i2c_functions_are_in_the_table_with_their_clocks() {
    // Gemini Lake, the HP 15s-fq0xxx's SoC: I2C0..I2C7 at 133 MHz.
    for id in (0x31ACu16..=0x31BA).step_by(2) {
        assert_eq!(device_info(id).map(|i| i.1), Some(CLOCK_HZ), "{id:#06x}");
    }
    // UART and SPI functions of the same LPSS blocks (Gemini Lake, Apollo
    // Lake, Sunrise Point-LP, Tiger Lake-LP) must never be taken for I2C.
    for id in
        [0x31BC, 0x31C2, 0x5ABC, 0x5AC2, 0x5AC4, 0x5AC6, 0x5AEE, 0x9D27, 0x9D29, 0xA0A8, 0xA0DA]
    {
        assert!(device_info(id).is_none(), "{id:#06x} is not an I2C function");
    }
    assert_eq!(device_info(0x5AAC).map(|i| i.1), Some(133_000_000));
    assert_eq!(device_info(0x9D60).map(|i| i.1), Some(120_000_000));
    assert_eq!(device_info(0x9DE8).map(|i| i.1), Some(216_000_000));
    assert_eq!(device_info(0x02E9).map(|i| i.1), Some(216_000_000));
}

#[test]
fn firmware_i2c_names_map_to_pci_functions_on_every_listed_platform() {
    assert_eq!(controller_index(0x31AC), Some(0));
    assert_eq!(controller_index(0x31B4), Some(4));
    assert_eq!(controller_index(0x31BA), Some(7));
    assert_eq!(controller_index(0x31BC), None);
    assert_eq!(controller_index(0x9D61), Some(1));
    assert_eq!(controller_index(0x9DC6), Some(5));
    assert_eq!(controller_index(0xA0D9), Some(7));
    assert_eq!(controller_index(0x51C5), Some(4));
    assert_eq!(controller_index(0x7E7B), Some(3));
}
