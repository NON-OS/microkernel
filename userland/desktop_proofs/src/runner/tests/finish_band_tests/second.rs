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

//! A second partial repaint changes no pixel.

use super::super::super::chrome::quarters;
use super::rig::{repaint, FRAME, H, W};
use nonos_toolkit::decorations::frame_rect_at;

#[test]
fn a_second_partial_repaint_changes_no_pixel() {
    let mut px = vec![0u32; (W * H) as usize];
    // The frame as a full paint left it: its rect in the frame colour.
    let f = frame_rect_at(W, H, false, quarters());
    for y in f.y..f.y + f.h {
        for x in f.x..f.x + f.w {
            px[(y * W + x) as usize] = FRAME;
        }
    }
    repaint(&mut px, false);
    let once = px.clone();
    for _ in 0..4 {
        repaint(&mut px, false);
    }
    if let Some(i) = (0..px.len()).find(|&i| px[i] != once[i]) {
        let (x, y) = (i as u32 % W, i as u32 / W);
        panic!("pixel ({x}, {y}) went from {:#010x} to {:#010x}", once[i], px[i]);
    }
}
