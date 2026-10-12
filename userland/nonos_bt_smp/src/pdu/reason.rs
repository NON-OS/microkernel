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

//! Pairing Failed reasons (Vol 3, Part H, 3.5.5).

pub const PASSKEY_ENTRY_FAILED: u8 = 0x01;
pub const OOB_NOT_AVAILABLE: u8 = 0x02;
pub const AUTHENTICATION_REQUIREMENTS: u8 = 0x03;
pub const CONFIRM_VALUE_FAILED: u8 = 0x04;
pub const PAIRING_NOT_SUPPORTED: u8 = 0x05;
pub const ENCRYPTION_KEY_SIZE: u8 = 0x06;
pub const COMMAND_NOT_SUPPORTED: u8 = 0x07;
pub const UNSPECIFIED_REASON: u8 = 0x08;
pub const REPEATED_ATTEMPTS: u8 = 0x09;
pub const INVALID_PARAMETERS: u8 = 0x0a;
pub const DHKEY_CHECK_FAILED: u8 = 0x0b;
pub const NUMERIC_COMPARISON_FAILED: u8 = 0x0c;
