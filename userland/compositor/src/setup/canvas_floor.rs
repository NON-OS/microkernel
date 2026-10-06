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
//! The smallest canvas clients are given. Setup's screens, run through
//! setup_layout_proofs' checks at 1024x720, fit there but for the
//! installer's footer hints, 12 pixels too wide, and do not at 800x600.
//! A firmware mode below 1024x720 (800x600, 640x480, a 1024x600 netbook
//! panel) cut them off. Such a screen gets a canvas of its own shape that
//! covers 1024x720, shrunk onto it when presented: soft, but every screen
//! is whole and the desktop still shows.

pub const FLOOR_WIDTH: u32 = 1024;
pub const FLOOR_HEIGHT: u32 = 720;

/// The canvas for a `width` x `height` screen smaller than the floor either
/// way, or None when the screen itself is big enough.
pub fn floor_canvas(width: u32, height: u32) -> Option<(u32, u32)> {
    if width == 0 || height == 0 || (width >= FLOOR_WIDTH && height >= FLOOR_HEIGHT) {
        return None;
    }
    let (w, h) = (width as u64, height as u64);
    // The larger of the two ratios, so both sides reach the floor.
    let (num, den) = if FLOOR_WIDTH as u64 * h >= FLOOR_HEIGHT as u64 * w {
        (FLOOR_WIDTH as u64, w)
    } else {
        (FLOOR_HEIGHT as u64, h)
    };
    let cw = (w * num).div_ceil(den);
    let ch = (h * num).div_ceil(den);
    Some((u32::try_from(cw).ok()?, u32::try_from(ch).ok()?))
}
