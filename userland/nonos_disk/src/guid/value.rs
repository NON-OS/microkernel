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

//! The sixteen bytes exactly as they sit in a partition entry.
//!
//! The first three fields are little-endian and the last two big-endian, so
//! the byte order on disk is not the order in the printed form. The ESP type
//! is spelled here in on-disk order and printed by `text` in canonical
//! order, so a reader can check both against the specification without
//! doing the swap in their head.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    /// The EFI system partition type, C12A7328-F81F-11D2-BA4B-00A0C93EC93B.
    pub const ESP: Guid = Guid([
        0x28, 0x73, 0x2A, 0xC1, 0x1F, 0xF8, 0xD2, 0x11, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9,
        0x3B,
    ]);

    /// A version-4 GUID from sixteen random bytes. The version and variant
    /// bits are forced, so the value is a well-formed random GUID whatever
    /// came in, and two disks written from different entropy never collide
    /// on the field a firmware menu keys on.
    pub fn from_random(mut bytes: [u8; 16]) -> Guid {
        // time_hi_and_version is little-endian at 6..8: version in the high
        // nibble of byte 7. clock_seq_hi is byte 8: variant bits 10xx.
        bytes[7] = (bytes[7] & 0x0F) | 0x40;
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        Guid(bytes)
    }
}
