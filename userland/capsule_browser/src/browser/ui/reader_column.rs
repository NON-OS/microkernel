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

use super::reader_geom::{MARGIN, PANEL_W};
use super::settings_cells::px;

pub const MIN_COLUMN: i32 = 420;
const SIDE: i32 = 40;
const WIDTHS: [i32; 3] = [560, 680, 820];

/// The narrowest content area the card sits beside the column in, rather
/// than over it, at the display scale.
pub fn wide_min() -> i32 {
    card_room() + 2 * SIDE + MIN_COLUMN
}

/* The width the card and its margins take at the display scale. */
fn card_room() -> i32 {
    px((PANEL_W + 2 * MARGIN) as u32)
}

/// The article column in a `w` wide content area, as `(x, width)`: the
/// chosen width held inside the room left of the card, centred in it. The
/// card sits beside the column while that leaves it `MIN_COLUMN`, and over
/// it only in an area narrower than `wide_min`.
pub fn column(w: i32, width: u8) -> (i32, i32) {
    let room = if w >= wide_min() { w - card_room() } else { w };
    let cw = WIDTHS[(width as usize).min(2)].min(room - 2 * SIDE).max(0);
    ((room - cw) / 2, cw)
}
