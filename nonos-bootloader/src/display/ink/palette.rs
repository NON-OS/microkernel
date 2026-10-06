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

//! The NØNOS palette for dark grounds, from nonos.software/brand: Ink black,
//! Paper text at three strengths, thin rules, and NØNOS Cyan as the one light.
//! Red and amber appear only where a boot is refused or a check is missing.

pub const GROUND: u32 = 0xFF000000;
/// The faint lift at the foot of the screen.
pub const GROUND_FOOT: u32 = 0xFF08090C;
pub const SURFACE: u32 = 0xFF0A0B0D;
pub const RAISED: u32 = 0xFF101215;
/// Paper at 12%: rules and outlines.
pub const BORDER: u32 = 0xFF1E1F21;
pub const HILITE: u32 = 0xFF2A2B2E;
pub const TEXT: u32 = 0xFFF2F3F7;
pub const TEXT_2: u32 = 0xFFB3B4B7;
pub const TEXT_3: u32 = 0xFF838386;
pub const CYAN: u32 = 0xFF66FFFF;
pub const ACCENT: u32 = CYAN;
/// Cyan at a fifth, for fills under a selection.
pub const ACCENT_SOFT: u32 = 0xFF0C1E1F;
pub const DEEP_TEAL: u32 = 0xFF2E5C5C;
pub const OK: u32 = CYAN;
pub const WARN: u32 = 0xFFFFB547;
pub const BAD: u32 = 0xFFFF5A5F;
pub const BAD_SOFT: u32 = 0xFF1F0B0C;
