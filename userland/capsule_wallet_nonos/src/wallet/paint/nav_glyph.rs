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

//! The shapes themselves, drawn from rectangles.

use nonos_app_skeleton::PaintBuffer;

use super::nav_icon::BOX;

// A roof over a body: two stepped courses for the pitch, then the walls.
pub(super) fn home(fb: &mut PaintBuffer, x: u32, y: u32, c: u32) {
    let mid = BOX / 2;
    for i in 0..mid {
        fb.fill_rect(x + mid - i - 1, y + i, (i + 1) * 2, 1, c);
    }
    fb.fill_rect(x + 2, y + mid, BOX - 4, BOX - mid - 2, c);
}

// A stem with a head, pointing down to receive and up to send.
pub(super) fn arrow(fb: &mut PaintBuffer, x: u32, y: u32, c: u32, down: bool) {
    let mid = BOX / 2;
    fb.fill_rect(x + mid - 1, y + 1, 2, BOX - 2, c);
    for i in 0..mid - 1 {
        let w = (mid - 1 - i) * 2;
        let ry = if down { y + BOX - 2 - i } else { y + 1 + i };
        fb.fill_rect(x + mid - (mid - 1 - i), ry, w, 1, c);
    }
}

pub(super) fn tick(fb: &mut PaintBuffer, x: u32, y: u32, c: u32) {
    let foot = BOX / 3;
    for i in 0..foot {
        fb.fill_rect(x + 1 + i, y + BOX / 2 + i, 2, 2, c);
    }
    for i in 0..BOX - foot - 1 {
        fb.fill_rect(x + foot + i, y + BOX / 2 + foot - i, 2, 2, c);
    }
}

// A crest that tapers to a point: full width at the top, closing to the tip.
pub(super) fn shield(fb: &mut PaintBuffer, x: u32, y: u32, c: u32) {
    let half = BOX / 2;
    for i in 0..BOX {
        let inset = if i < half { 0 } else { (i - half) * half / half };
        let w = BOX.saturating_sub(inset * 2);
        if w == 0 {
            break;
        }
        fb.fill_rect(x + inset, y + i, w, 1, c);
    }
}

// A ring, for the token rail.
pub(super) fn token(fb: &mut PaintBuffer, x: u32, y: u32, c: u32) {
    fb.fill_rect(x + 3, y, BOX - 6, 2, c);
    fb.fill_rect(x + 3, y + BOX - 2, BOX - 6, 2, c);
    fb.fill_rect(x, y + 3, 2, BOX - 6, c);
    fb.fill_rect(x + BOX - 2, y + 3, 2, BOX - 6, c);
}
