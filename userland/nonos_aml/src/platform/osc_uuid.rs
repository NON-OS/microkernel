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

//! The _OSC UUIDs NONOS passes.

/// The platform-wide capabilities UUID (6.2.11.2),
/// 0811B06E-4A27-44F9-8D60-3CBBC22E7B48, in ToUUID's byte order.
pub const PLATFORM_UUID: [u8; 16] = [
    0x6E, 0xB0, 0x11, 0x08, 0x27, 0x4A, 0xF9, 0x44, 0x8D, 0x60, 0x3C, 0xBB, 0xC2, 0x2E, 0x7B, 0x48,
];
/// The PCI host bridge UUID (PCI Firmware 3.3, 4.5.1),
/// 33DB4D5B-1FF7-401C-9657-7441C03DD766.
pub const PCI_HOST_UUID: [u8; 16] = [
    0x5B, 0x4D, 0xDB, 0x33, 0xF7, 0x1F, 0x1C, 0x40, 0x96, 0x57, 0x74, 0x41, 0xC0, 0x3D, 0xD7, 0x66,
];
