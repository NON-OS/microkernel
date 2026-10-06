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

//! One source of truth for desktop type sizes and where a line of text sits inside
//! a box. `ttf` clamps every draw and measure to `MIN_UI_PX`, so `UI_PX` is that
//! floor rather than a smaller size that would be silently ignored.

use core::sync::atomic::{AtomicU32, Ordering};
use nonos_toolkit::font::ttf;

/// Drawing pixels per logical pixel, in quarters: 4 is one to one.
static QUARTERS: AtomicU32 = AtomicU32::new(4);

/// Latch the display scale, in quarters of a pixel. Every size below is a
/// logical size taken through `px` or `scaled`, so the desktop's type and its
/// bars, icons and menus grow together, by the same rule first-boot setup and
/// the installer use. Set once from the surface geometry before the first
/// frame.
pub fn set_scale(quarters: u32) {
    QUARTERS.store(quarters.max(4), Ordering::Relaxed);
}

/// `v` logical pixels on the canvas, to the nearest pixel.
pub fn px(v: u32) -> u32 {
    v.saturating_mul(QUARTERS.load(Ordering::Relaxed)).saturating_add(2) / 4
}

/// Whole drawing pixels per logical pixel, for the glyphs drawn in whole
/// pixels (the status icons, the menu marks): 1 up to a scale of 1.5, 2 from
/// 2. A glyph's metrics use this too, so the room kept for it is the room it
/// takes.
pub fn scale() -> u32 {
    (QUARTERS.load(Ordering::Relaxed) / 4).max(1)
}

/// Convert a logical point size to device pixels. Draw and measure both route
/// through here, so they cannot disagree about how large a glyph is.
pub fn scaled(px: f32) -> f32 {
    px * QUARTERS.load(Ordering::Relaxed) as f32 / 4.0
}

/// Body text: menu bar, status cluster, labels, dialogs.
pub const UI_PX: f32 = ttf::MIN_UI_PX;

/// Headings: the launcher title.
pub const TITLE_PX: f32 = 20.0;

/// The brand wordmark, set in the bold face.
pub const BRAND_PX: f32 = 14.0;

/// Menu bar titles.
pub const MENU_PX: f32 = 13.5;

/// The right-hand status cluster and its clock.
pub const STATUS_PX: f32 = 13.0;

/// Desktop icon names.
pub const LABEL_PX: f32 = 12.5;

/// Captions: the second, dimmer line under a desktop icon name.
pub const META_PX: f32 = 11.0;

/// Height of one line box at `px`.
pub fn line_h(px: f32) -> u32 {
    ttf::line_height(scaled(px)).max(0) as u32
}

/// Top edge that centres one line of `px` text inside a box of `box_h`.
pub fn top_y_centered(box_y: u32, box_h: u32, px: f32) -> u32 {
    box_y + box_h.saturating_sub(line_h(px)) / 2
}

/// The valid UTF-8 prefix of `bytes`, so malformed state renders short instead of blank.
pub fn valid_str(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes)
        .unwrap_or_else(|e| core::str::from_utf8(&bytes[..e.valid_up_to()]).unwrap_or(""))
}
