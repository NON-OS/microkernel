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

//! Capabilities register fields and Host Controller Version values.

// Capabilities (0x40).
pub const CAP_BASE_CLK_SHIFT: u32 = 8;
pub const CAP_BASE_CLK_MASK_V3: u32 = 0xff;
pub const CAP_BASE_CLK_MASK_V2: u32 = 0x3f;
pub const CAP_8BIT: u32 = 1 << 18;
pub const CAP_ADMA2: u32 = 1 << 19;
pub const CAP_HIGH_SPEED: u32 = 1 << 21;
pub const CAP_330: u32 = 1 << 24;
pub const CAP_300: u32 = 1 << 25;
pub const CAP_180: u32 = 1 << 26;
pub const CAP_64BIT_V3: u32 = 1 << 28;
pub const CAP_SLOT_TYPE_SHIFT: u32 = 30;
pub const SLOT_TYPE_EMBEDDED: u8 = 1;

pub const SPEC_200: u8 = 1;
pub const SPEC_300: u8 = 2;
