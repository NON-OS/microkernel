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

//! The NØNOS palette on dark grounds, from nonos.software/brand, the same
//! values the bootloader's screens use (nonos-bootloader/src/display/ink).

pub const GROUND: u32 = 0xFF000000;
pub const INK: u32 = 0xFF0A0B0D;
pub const CARD: u32 = 0xFF101215;
/// Paper at 12%: rules and outlines.
pub const RULE: u32 = 0xFF1E1F21;
pub const TEXT: u32 = 0xFFF2F3F7;
pub const TEXT_2: u32 = 0xFFB3B4B7;
pub const TEXT_3: u32 = 0xFF838386;
pub const CYAN: u32 = 0xFF66FFFF;
/// Cyan at a fifth, under a selection.
pub const CYAN_SOFT: u32 = 0xFF0C1E1F;
pub const WARN: u32 = 0xFFFFB547;
pub const BAD: u32 = 0xFFFF5A5F;
