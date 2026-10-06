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

//! A panel: a fill inside a one pixel edge, lit along the top, with a short
//! soft shadow under it.

use super::palette::HILITE;
use super::shape::round_rect;
use crate::display::fx::mix;
use crate::display::gop::{fill_rect, get_pixel, put_pixel};

/// A slab, the brand's panel: a near-square fill inside a one pixel edge,
/// lit along the top, with a short soft shadow under it.
pub fn card(x: u32, y: u32, w: u32, h: u32, r: u32, fill: u32, border: u32) {
    let r = r.min(4);
    for i in 0..6u32 {
        let a = (6 - i) * 14;
        for px in x + r..(x + w).saturating_sub(r) {
            let py = y + h + i;
            put_pixel(px, py, mix(get_pixel(px, py), 0xFF00_0000, a));
        }
    }
    round_rect(x, y, w, h, r, border);
    round_rect(x + 1, y + 1, w.saturating_sub(2), h.saturating_sub(2), r.saturating_sub(1), fill);
    fill_rect(x + r, y, w.saturating_sub(2 * r), 1, HILITE);
}
