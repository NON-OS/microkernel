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

//! The values EXT_CSD fields take: device type bits, bus width, partition.

/// DEVICE_TYPE bit 0: High Speed at 26 MHz; bit 1: at 52 MHz.
pub const TYPE_HS_26: u8 = 1 << 0;
pub const TYPE_HS_52: u8 = 1 << 1;

/// BUS_WIDTH values for SDR 1, 4 and 8 data lines.
pub const fn bus_width_value(bits: u8) -> u8 {
    match bits {
        8 => 2,
        4 => 1,
        _ => 0,
    }
}

/// PARTITION_CONFIG bits 2:0, PARTITION_ACCESS: 0 is the user data area.
pub const PARTITION_ACCESS_MASK: u8 = 0x7;
