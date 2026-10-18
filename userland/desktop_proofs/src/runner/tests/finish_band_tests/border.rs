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

//! The border beside the band is left alone.

use super::super::super::chrome::quarters;
use super::rig::{repaint, FRAME, H, W};
use nonos_toolkit::decorations::content_rect_at;

#[test]
fn the_border_beside_the_band_is_left_alone() {
    let mut px = vec![FRAME; (W * H) as usize];
    let c = content_rect_at(W, H, false, quarters());
    repaint(&mut px, false);
    for y in 0..H {
        for x in 0..W {
            let inside = x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + c.h;
            if !inside {
                assert_eq!(px[(y * W + x) as usize], FRAME, "({x}, {y}) is outside the content");
            }
        }
    }
}
