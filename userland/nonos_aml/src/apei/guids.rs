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

//! The section type GUIDs (UEFI 2.10, N.2.2), as they lie in memory: the
//! first three fields little-endian.

/// 9876CCAD-47B4-4BDB-B65E-16F193C4F3DB.
pub const PROCESSOR: [u8; 16] = [
    0xAD, 0xCC, 0x76, 0x98, 0xB4, 0x47, 0xDB, 0x4B, 0xB6, 0x5E, 0x16, 0xF1, 0x93, 0xC4, 0xF3, 0xDB,
];
/// A5BC1114-6F64-4EDE-B863-3E83ED7C83B1.
pub const MEMORY: [u8; 16] = [
    0x14, 0x11, 0xBC, 0xA5, 0x64, 0x6F, 0xDE, 0x4E, 0xB8, 0x63, 0x3E, 0x83, 0xED, 0x7C, 0x83, 0xB1,
];
/// D995E954-BBC1-430F-AD91-B44DCB3C6F35.
pub const PCIE: [u8; 16] = [
    0x54, 0xE9, 0x95, 0xD9, 0xC1, 0xBB, 0x0F, 0x43, 0xAD, 0x91, 0xB4, 0x4D, 0xCB, 0x3C, 0x6F, 0x35,
];
