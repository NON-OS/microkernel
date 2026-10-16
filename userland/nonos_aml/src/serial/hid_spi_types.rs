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

//! What HID over SPI's _DSM gives (`hid_spi`), and which devices speak it.

use crate::platform::HardwareId;

/// 6E2AC436-0FCF-41AF-A265-B32A220DCFAB, in ToUUID's byte order.
pub const HID_SPI_DSM_UUID: [u8; 16] = [
    0x36, 0xC4, 0x2A, 0x6E, 0xCF, 0x0F, 0xAF, 0x41, 0xA2, 0x65, 0xB3, 0x2A, 0x22, 0x0D, 0xCF, 0xAB,
];
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HidSpi {
    pub input_header: Option<u32>,
    pub input_body: Option<u32>,
    pub output: Option<u32>,
    pub read_opcode: Option<u8>,
    pub write_opcode: Option<u8>,
    pub flags: Option<u16>,
}

pub fn is_hid_over_spi(ids: &[HardwareId]) -> bool {
    ids.iter().any(|h| h.bytes() == b"PNP0C51")
}
