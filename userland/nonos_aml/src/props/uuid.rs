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

//! The two _DSD UUIDs NONOS reads, in ToUUID's byte order. Both are the
//! UEFI Forum's, published beside the specification (the "_DSD Device
//! Properties UUID" and "_DSD Hierarchical Data Extension UUID" documents);
//! that site cannot be reached from here, so both are recalled: unverified.

/// daffd814-6eba-4d8c-8a91-bc9bbf4aa301.
pub const DEVICE_PROPERTIES_UUID: [u8; 16] = [
    0x14, 0xD8, 0xFF, 0xDA, 0xBA, 0x6E, 0x8C, 0x4D, 0x8A, 0x91, 0xBC, 0x9B, 0xBF, 0x4A, 0xA3, 0x01,
];

/// dbb8e3e6-5886-4ba6-8795-1319f52a966b.
pub const HIERARCHICAL_DATA_UUID: [u8; 16] = [
    0xE6, 0xE3, 0xB8, 0xDB, 0x86, 0x58, 0xA6, 0x4B, 0x87, 0x95, 0x13, 0x19, 0xF5, 0x2A, 0x96, 0x6B,
];
