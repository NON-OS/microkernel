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

//! Cipher and AKM suite selectors (IEEE Std 802.11-2020, Tables 9-149 and
//! 9-151). A selector is the 3-octet OUI then a type octet; here it is held as
//! one big-endian u32 so a suite read off the air compares with one constant.

/// The IEEE 802.11 OUI, 00-0F-AC.
pub const OUI_IEEE: [u8; 3] = [0x00, 0x0F, 0xAC];

/// A suite selector from its four octets as they appear on the air.
pub const fn selector(b: [u8; 4]) -> u32 {
    u32::from_be_bytes(b)
}

// Cipher suites.
pub const CIPHER_TKIP: u32 = selector([0x00, 0x0F, 0xAC, 2]);
pub const CIPHER_CCMP: u32 = selector([0x00, 0x0F, 0xAC, 4]);
pub const CIPHER_BIP_CMAC_128: u32 = selector([0x00, 0x0F, 0xAC, 6]);

// AKM suites.
pub const AKM_PSK: u32 = selector([0x00, 0x0F, 0xAC, 2]);
pub const AKM_PSK_SHA256: u32 = selector([0x00, 0x0F, 0xAC, 6]);
pub const AKM_SAE: u32 = selector([0x00, 0x0F, 0xAC, 8]);

// RSN Capabilities bits (IEEE Std 802.11-2020, Figure 9-342).
/// Management frame protection required.
pub const CAP_MFPR: u16 = 1 << 6;
/// Management frame protection capable.
pub const CAP_MFPC: u16 = 1 << 7;
