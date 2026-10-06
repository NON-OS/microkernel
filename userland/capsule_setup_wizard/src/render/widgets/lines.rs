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

use crate::render::ink::draw_text;
use crate::render::layout::layout;

/// `lines` one under another from (x, y), a body line apart; returns the y
/// below the last.
pub fn text(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    lines: &[&[u8]],
    color: u32,
) -> u32 {
    let step = layout(w, h).line_h;
    let mut yy = y;
    for line in lines {
        draw_text(buf, spx, w, h, x, yy, line, color);
        yy += step;
    }
    yy
}
