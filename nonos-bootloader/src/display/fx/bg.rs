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

use crate::display::ink::palette::{GROUND, GROUND_FOOT};

/*
 * The ground: Ink black, lifting to GROUND_FOOT over the last eighth of the
 * screen, with a 4x4 ordered dither so the lift has no bands. Pure and
 * integer only, so it is written once per pixel with no framebuffer reads.
 */
const BAYER: [[i32; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

pub(super) fn bg_at(x: u32, y: u32, _w: u32, h: u32) -> u32 {
    let start = h as i32 * 7 / 8;
    let t = ((y as i32 - start).max(0) * 256 / (h as i32 - start).max(1)).min(256);
    if t == 0 {
        return GROUND;
    }
    let d = BAYER[(y % 4) as usize][(x % 4) as usize];
    let ch = |shift: u32| {
        let (a, b) = (((GROUND >> shift) & 0xFF) as i32, ((GROUND_FOOT >> shift) & 0xFF) as i32);
        let v16 = (a * 16 * (256 - t) + b * 16 * t) / 256 + d - 8;
        ((v16.max(0) / 16).min(255) as u32) << shift
    };
    0xFF00_0000 | ch(16) | ch(8) | ch(0)
}
