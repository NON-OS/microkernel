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

//! Descriptors for the model, laid out by USB 3.2 chapter 9 with what
//! r8152 asks of the vendor configuration: a first interface of class
//! 0xFF with bulk IN on endpoint 1, bulk OUT on endpoint 2 and interrupt
//! IN on endpoint 3 (rtl_check_vendor_ok). The vendor configuration is
//! the second, after one whose first interface is CDC communications,
//! because r8152 notes the vendor mode is not always configuration 1.
//! Built for the proofs, not dumped from a device.

/// A SuperSpeed device descriptor with two configurations.
pub fn device(ids: (u16, u16)) -> [u8; 18] {
    let (v, p) = (ids.0.to_le_bytes(), ids.1.to_le_bytes());
    [18, 0x01, 0x00, 0x03, 0, 0, 0, 9, v[0], v[1], p[0], p[1], 0x00, 0x30, 1, 2, 0, 2]
}

/// bConfigurationValue 1: a CDC communications interface first.
pub const CONFIG_COMM: [u8; 18] = [
    9, 0x02, 18, 0, 1, 1, 0, 0xa0, 0x32, //
    9, 0x04, 0, 0, 0, 0x02, 0x06, 0x00, 0,
];

/// bConfigurationValue 2: the vendor interface and its three endpoints,
/// each with its SuperSpeed companion (bursts of 4 on the bulk pair).
pub const CONFIG_VENDOR: [u8; 57] = [
    9, 0x02, 57, 0, 1, 2, 0, 0xa0, 0x32, //
    9, 0x04, 0, 0, 3, 0xff, 0xff, 0x00, 0, //
    7, 0x05, 0x81, 0x02, 0x00, 0x04, 0, //
    6, 0x30, 3, 0, 0, 0, //
    7, 0x05, 0x02, 0x02, 0x00, 0x04, 0, //
    6, 0x30, 3, 0, 0, 0, //
    7, 0x05, 0x83, 0x03, 0x02, 0x00, 8, //
    6, 0x30, 0, 0, 2, 0,
];
