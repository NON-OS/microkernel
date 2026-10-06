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

//! What the window shows: ink, a cyan frame and band, and a ring.

const INK: u32 = 0xFF0A_0B0D;
const CYAN: u32 = 0xFF66_FFFF;
const TEAL: u32 = 0xFF2E_5C5C;

pub fn paint(px: &mut [u32], w: usize, h: usize) {
    let (cx, cy, r) = (w as i64 / 2, h as i64 / 2 + 16, (h.min(w) / 4) as i64);
    for y in 0..h {
        for x in 0..w {
            let edge = x < 3 || y < 3 || x >= w - 3 || y >= h - 3;
            let band = y < 40;
            let (dx, dy) = (x as i64 - cx, y as i64 - cy);
            let d2 = dx * dx + dy * dy;
            let ring = d2 <= r * r && d2 >= (r - 10) * (r - 10);
            // The slash of the Ø, a band along the diagonal inside the ring.
            let slash = d2 <= r * r && (dx + dy).abs() <= 6;
            px[y * w + x] = if edge || ring || slash {
                CYAN
            } else if band {
                TEAL
            } else {
                INK
            };
        }
    }
}
