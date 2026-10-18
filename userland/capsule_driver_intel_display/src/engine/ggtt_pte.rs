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

//! The global GTT's entries, in the two formats the display engines this
//! driver takes over read. Each maps one 4 KiB page of the capsule's IOVA
//! space, since the engine's reads pass through the graphics DMAR unit.

/// The entry format, and where the table sits in GTTMMADR.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pte {
    /// Haswell: 32-bit entries from 2 MiB, address bits 31:12 in 31:12 and
    /// 39:32 in 11:4, valid in bit 0, cacheability 3:1 left 0 (HSW PRM
    /// Vol 2c, GTT entry format; positions unverified).
    Narrow,
    /// Broadwell on: 64-bit entries from 8 MiB, address bits 38:12 in
    /// place, present in bit 0 (offset unverified against GTTMMADR's
    /// description).
    Wide,
}

impl Pte {
    pub fn table(self) -> u32 {
        match self {
            Pte::Narrow => 0x20_0000,
            Pte::Wide => 0x80_0000,
        }
    }

    /// Bytes per entry.
    pub fn size(self) -> u32 {
        match self {
            Pte::Narrow => 4,
            Pte::Wide => 8,
        }
    }

    /// The highest IOVA bit plus one an entry can carry.
    pub fn reach(self) -> u64 {
        match self {
            Pte::Narrow => 1 << 40,
            Pte::Wide => 1 << 39,
        }
    }

    /// The entry mapping page `iova` (4 KiB aligned, below `reach`).
    pub fn encode(self, iova: u64) -> u64 {
        match self {
            Pte::Narrow => (iova & 0xFFFF_F000) | ((iova >> 28) & 0xFF0) | 1,
            Pte::Wide => (iova & 0x7F_FFFF_F000) | 1,
        }
    }
}
