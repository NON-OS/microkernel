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

//! SMP command codes (Vol 3, Part H, 3.3) and each one's parameter length
//! (Vol 3, Part H, 3.5 and 3.6).

pub const PAIRING_REQUEST: u8 = 0x01;
pub const PAIRING_RESPONSE: u8 = 0x02;
pub const PAIRING_CONFIRM: u8 = 0x03;
pub const PAIRING_RANDOM: u8 = 0x04;
pub const PAIRING_FAILED: u8 = 0x05;
pub const ENCRYPTION_INFORMATION: u8 = 0x06;
pub const CENTRAL_IDENTIFICATION: u8 = 0x07;
pub const IDENTITY_INFORMATION: u8 = 0x08;
pub const IDENTITY_ADDRESS_INFORMATION: u8 = 0x09;
pub const SIGNING_INFORMATION: u8 = 0x0a;
pub const SECURITY_REQUEST: u8 = 0x0b;
pub const PAIRING_PUBLIC_KEY: u8 = 0x0c;
pub const PAIRING_DHKEY_CHECK: u8 = 0x0d;
pub const KEYPRESS_NOTIFICATION: u8 = 0x0e;

/// The parameter length of `code`, `None` for a code SMP does not define.
pub fn params_len(code: u8) -> Option<usize> {
    Some(match code {
        PAIRING_REQUEST | PAIRING_RESPONSE => 6,
        PAIRING_CONFIRM | PAIRING_RANDOM | PAIRING_DHKEY_CHECK => 16,
        ENCRYPTION_INFORMATION | IDENTITY_INFORMATION | SIGNING_INFORMATION => 16,
        PAIRING_FAILED | SECURITY_REQUEST | KEYPRESS_NOTIFICATION => 1,
        CENTRAL_IDENTIFICATION => 10,
        IDENTITY_ADDRESS_INFORMATION => 7,
        PAIRING_PUBLIC_KEY => 64,
        _ => return None,
    })
}
