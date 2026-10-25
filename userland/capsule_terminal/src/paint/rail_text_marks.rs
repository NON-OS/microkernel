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

//! The rail's section captions and level bars.

use nonos_app_skeleton::PaintBuffer;

use super::{band_rect, left, lh, BAR_ALPHA, BAR_H, RAIL_GAP};
use crate::paint::shade::elevate;
use crate::term::theme::types::Theme;

/// A section caption with the hairline that separates it from the block above,
/// returning the first content row so no caller re-derives the stack.
pub fn head(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, label: &str, t: &Theme) -> i32 {
    band_rect(fb, x, y, w, 1, t.chrome_edge);
    let top = y + RAIL_GAP as i32;
    left(fb, x, top, label, t.dim);
    top + (lh() + RAIL_GAP / 2) as i32
}

/// A track with the filled share drawn over it. Both are blends: the rail sits
/// on pixels the frame already painted, so a raw write would punch through.
///
/// The track barely lifts off the background and the fill is carried at less
/// than full opacity, so the bar reads as a level rather than a hard rule cut
/// across the rail.
pub fn bar(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, pct: u32, t: &Theme) {
    band_rect(fb, x, y, w, BAR_H, elevate(t.bg, 6));
    let fill = w * pct.min(100) / 100;
    if fill > 0 {
        band_rect(fb, x, y, fill, BAR_H, (t.accent & 0x00FF_FFFF) | BAR_ALPHA);
    }
}
