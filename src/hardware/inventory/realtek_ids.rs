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

//! The Realtek network parts a capsule starts for, the capsules' own ids
//! (kernel_proofs holds them together): the 8139, the 8169 family's
//! Ethernet parts, and the RTL8821CE Wi-Fi.
//!
//! Pure, with no imports, so host proofs can include it by path.

pub const RTL8139_DEVICE_ID: u16 = 0x8139;
pub const RTL8169_DEVICE_IDS: [u16; 10] =
    [0x2502, 0x2600, 0x3000, 0x8125, 0x8136, 0x8161, 0x8162, 0x8167, 0x8168, 0x8169];
pub const RTL8821CE_DEVICE_ID: u16 = 0xc821;
