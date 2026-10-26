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
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
//! chrome). Each grows by the display scale through scale.rs.

pub const BORDER_PX: u32 = 1;
pub const HAIRLINE_PX: u32 = 1;
pub const SHADOW_MARGIN: u32 = 10;
/// Smaller than a tile's corners: windows are tools, not tiles.
pub const FRAME_RADIUS: u32 = 10;
pub const TITLEBAR_H: u32 = 36;
/// `type.body`, the least the toolkit draws while `ttf::MIN_UI_PX` is 17.
pub const TITLE_PX: f32 = 17.0;
/// From the frame's left edge to the title.
pub const TITLE_INSET: u32 = 16;
/// Each control's target: minimize, maximize, close, on the right.
pub const CONTROL_W: u32 = 36;
pub const CONTROL_H: u32 = 28;
/// From the frame's right edge to the close control's target.
pub const CONTROL_INSET: u32 = 4;
/// Half the width of a control's line glyph, which sits in a 16 px box.
pub const GLYPH_ARM: u32 = 5;
/// The glyph's stroke, in quarters of a pixel: 1.5 px at one to one.
pub const GLYPH_STROKE_Q: u32 = 6;
pub const ACCESSORY_INSET: u32 = 12;
pub const ACCESSORY_PAD_Y: u32 = 7;
