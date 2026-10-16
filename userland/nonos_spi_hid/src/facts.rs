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
//! Microsoft's HID over SPI protocol specification, v1.0, as recalled,
//! every value unverified (the letter's verification rows list each). The
//! device answers an interrupt with an input report: a 4-byte header, then
//! a body of report type, content length and content id; the host writes
//! output reports of the same body shape. Each SPI transaction starts with
//! the opcode and 24-bit address the firmware's _DSM names.

/// The input report header's version field (bits 3:0), and its sync byte.
pub const HEADER_VERSION: u32 = 3;
pub const HEADER_SYNC: u8 = 0x5A;
/// The header's length field (bits 21:8) counts 4-byte words of body.
pub const LENGTH_SHIFT: u32 = 8;
pub const LENGTH_MASK: u32 = 0x3FFF;
/// Bit 22: this is the last fragment of the report.
pub const LAST_FRAGMENT: u32 = 1 << 22;
pub const HEADER_LEN: usize = 4;
/// A body's report type, content length le16 and content id.
pub const BODY_HEAD: usize = 4;

/// Input report types: what the device says.
pub const IN_DATA: u8 = 0x01;
pub const IN_RESET: u8 = 0x03;
pub const IN_DEVICE_DESCRIPTOR: u8 = 0x07;
pub const IN_REPORT_DESCRIPTOR: u8 = 0x08;
/// The device's answers to a get-feature and a set-feature request.
pub const IN_GET_FEATURE: u8 = 0x05;
pub const IN_SET_FEATURE: u8 = 0x09;

/// Output report types: what the host asks.
pub const OUT_DEVICE_DESCRIPTOR: u8 = 0x01;
pub const OUT_REPORT_DESCRIPTOR: u8 = 0x02;
pub const OUT_SET_FEATURE: u8 = 0x03;
pub const OUT_GET_FEATURE: u8 = 0x04;

/// The device descriptor's length, little-endian words throughout.
pub const DEVICE_DESCRIPTOR_LEN: usize = 24;
/// A frame's opcode byte and 24-bit address.
pub const FRAME_HEAD: usize = 4;
