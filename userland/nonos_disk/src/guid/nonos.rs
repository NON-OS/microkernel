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

//! The partition types NONOS gives its own partitions. Random version-4
//! GUIDs drawn once for this purpose and fixed here, in on-disk order; the
//! canonical text of each is beside it, as `gdisk` prints it.

use super::Guid;

impl Guid {
    /// NONOS package store, 94FB361A-D500-4798-936B-06EB4685AA8F.
    pub const NONOS_STORE: Guid = Guid([
        0x1A, 0x36, 0xFB, 0x94, 0x00, 0xD5, 0x98, 0x47, 0x93, 0x6B, 0x06, 0xEB, 0x46, 0x85, 0xAA,
        0x8F,
    ]);

    /// NONOS disk plan and key header, 6F5AE6D3-8FE5-4836-BA18-819E9FACC900.
    pub const NONOS_PLAN: Guid = Guid([
        0xD3, 0xE6, 0x5A, 0x6F, 0xE5, 0x8F, 0x36, 0x48, 0xBA, 0x18, 0x81, 0x9E, 0x9F, 0xAC, 0xC9,
        0x00,
    ]);

    /// NONOS data volume, 2FB2309E-8C9C-4DE4-A941-0161EB222B66.
    pub const NONOS_DATA: Guid = Guid([
        0x9E, 0x30, 0xB2, 0x2F, 0x9C, 0x8C, 0xE4, 0x4D, 0xA9, 0x41, 0x01, 0x61, 0xEB, 0x22, 0x2B,
        0x66,
    ]);
}
