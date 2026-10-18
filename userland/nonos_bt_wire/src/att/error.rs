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

//! ATT error codes (Vol 3, Part F, 3.4.1.1).

pub const INVALID_HANDLE: u8 = 0x01;
pub const READ_NOT_PERMITTED: u8 = 0x02;
pub const WRITE_NOT_PERMITTED: u8 = 0x03;
pub const INVALID_PDU: u8 = 0x04;
pub const INSUFFICIENT_AUTHENTICATION: u8 = 0x05;
pub const REQUEST_NOT_SUPPORTED: u8 = 0x06;
pub const INVALID_OFFSET: u8 = 0x07;
pub const INSUFFICIENT_AUTHORIZATION: u8 = 0x08;
pub const ATTRIBUTE_NOT_FOUND: u8 = 0x0a;
pub const ATTRIBUTE_NOT_LONG: u8 = 0x0b;
pub const INSUFFICIENT_KEY_SIZE: u8 = 0x0c;
pub const INVALID_ATTRIBUTE_VALUE_LENGTH: u8 = 0x0d;
pub const UNLIKELY_ERROR: u8 = 0x0e;
pub const INSUFFICIENT_ENCRYPTION: u8 = 0x0f;
pub const UNSUPPORTED_GROUP_TYPE: u8 = 0x10;
pub const INSUFFICIENT_RESOURCES: u8 = 0x11;
