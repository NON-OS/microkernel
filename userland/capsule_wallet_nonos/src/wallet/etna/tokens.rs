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

//! The Etna design tokens: the phones' design-tokens.json, as the framebuffer
//! takes them. Nothing on a wallet screen is drawn in any other colour or at
//! any other spacing, so a value missing here is a value the design lacks.

pub const INK: u32 = 0xFF0A_0B0D;
pub const SURFACE: u32 = 0xFF10_1215;
pub const TEXT: u32 = 0xFFF2_F3F7;
pub const TEXT_2: u32 = 0xFFA3_A4A7;
pub const TEXT_3: u32 = 0xFF87_888B;
pub const LINE: u32 = 0xFF1F_2022;
pub const LINE_2: u32 = 0xFF2F_3032;
pub const OUTLINE: u32 = 0xFF27_282A;
pub const FIELD: u32 = 0xFF16_1719;
pub const BANNER: u32 = 0xFF1E_1F21;
pub const CYAN: u32 = 0xFF66_FFFF;
pub const DEEP_TEAL: u32 = 0xFF2E_5C5C;
pub const BAD: u32 = 0xFFFF_8A8A;

// Room: the only spacing numbers there are.
pub const SIDE: u32 = 24;
pub const HALF: u32 = 12;
pub const ROOM: u32 = 28;
pub const GAP: u32 = 26;
pub const TIGHT: u32 = 12;

// Edge: nearly square controls, rounder tiles, one-pixel lines.
pub const CORNER: u32 = 2;
pub const RADIUS: u32 = 14;
pub const HAIR: u32 = 1;
pub const TALL: u32 = 58;
pub const ROUND: u32 = 56;

/// The column the phone's frame is laid in on a desktop, and so the width the
/// section photographs were sized to by tools/nonos-etna-banners.
pub const COLUMN: u32 = 560;
/// The photographs keep the phones' 1290 by 860.
pub const BANNER_H: u32 = COLUMN * 860 / 1290;
/// The back control is a 44 point square; the bar holds it with room above
/// and below.
pub const BACK: u32 = 44;
pub const BAR_H: u32 = BACK + TIGHT;
/// Mono 10 with 10 above and below.
pub const STATUS_H: u32 = 32;
