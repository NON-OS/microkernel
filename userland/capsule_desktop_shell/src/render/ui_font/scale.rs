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

//! The display scale every logical size is taken through.

use core::sync::atomic::{AtomicU32, Ordering};

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
