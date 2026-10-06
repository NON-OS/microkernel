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

//! The brand frame: the 9 by 10 tile outline with a soft glow, the Ø inside.

use nonos_app_skeleton::PaintBuffer;

use super::mark::mark;
use super::palette::GROUND;
use super::pixels::mix;

/// The emblem in the box at (x, y), `w` wide, in `argb`; the Ø lit or not.
pub fn emblem(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, argb: u32, lit: bool) {
    let h = w * 10 / 9;
    let r = w / 5;
    let glow = (w / 24).max(3);
    for i in (1..=glow).rev() {
        let a = (glow + 1 - i) * 60 / glow;
        fb.panel(x - i, y - i, w + 2 * i, h + 2 * i, r + i, GROUND, mix(GROUND, argb, a * a / 60));
    }
    fb.panel(x, y, w, h, r, GROUND, argb);
    fb.panel(x + 1, y + 1, w - 2, h - 2, r - 1, GROUND, argb);
    let (ink, glow) = if lit { (argb, 200) } else { (super::palette::TEXT_3, 0) };
    mark(fb, x + w / 2, y + h / 2, w * 2 / 5 * 23 / 20, ink, glow);
}
