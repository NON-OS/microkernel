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

use super::layout::{metrics, px};

// The rail's own metrics. Every plate on it -- an active row's pill, the brand
// mark, the drive card -- is inset by the same amount from both edges, so the
// column reads as one stack rather than as three differently-margined things.
const RAIL_X: u32 = 8;
const RAIL_PAD: u32 = 12;

// Icons are stroked line art: at 18px the fine detail collapsed, so the rail
// carries them at 22 and the label column starts past that.
const ICON_S: u32 = 22;
const ICON_GAP: u32 = 12;

/// The rail's metrics at the display scale in force, in drawing pixels: the
/// plate inset `x` and inner `pad`, the icon side, the width of a full-bleed
/// plate and the left edge of the content inside one.
pub struct Rail {
    pub x: u32,
    pub pad: u32,
    pub icon_s: u32,
    pub w: u32,
    pub text_x: u32,
}

/// The rail at the display scale in force: each logical size above read once.
pub fn rail() -> Rail {
    let (x, pad, icon_s) = (px(RAIL_X), px(RAIL_PAD), px(ICON_S));
    let w = metrics().sidebar_w.saturating_sub(x * 2);
    Rail { x, pad, icon_s, w, text_x: x + pad + icon_s + px(ICON_GAP) }
}

/// The height a single line of `px` text occupies, used to centre a run against
/// an icon of a known size. The toolkit's real line metrics are not exported to
/// capsules, so this is the same 1.25em box the rest of the chrome assumes.
pub fn line_box(px: f32) -> u32 {
    let clamped = if px < MIN_PX { MIN_PX } else { px };
    (clamped * 1.25) as u32
}

/// The floor the font layer clamps every draw and every measure up to. Stated
/// here because layout that ignores it centres text against the size it asked
/// for rather than the size that lands on screen.
pub const MIN_PX: f32 = 17.0;
