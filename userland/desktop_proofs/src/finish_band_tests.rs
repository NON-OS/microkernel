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

//! A partial repaint redraws the content and the rounded corners over the
//! rows it drew. The border beside those rows is not redrawn, so it must be
//! left as it is: blended again on every repaint, the corners' edge pixels
//! went more opaque and darker with each line a terminal printed.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::{content_rect_at, frame_rect_at};

use super::chrome::quarters;
use super::finish_band::finish_band;

const W: u32 = 160;
const H: u32 = 120;
const FRAME: u32 = 0xFF2A_2E36;
const CONTENT: u32 = 0xFF33_6699;

/// The app's content paint: its whole rect, nothing outside it.
fn paint_content(px: &mut [u32], maximized: bool) {
    let c = content_rect_at(W, H, maximized, quarters());
    for y in c.y..c.y + c.h {
        for x in c.x..c.x + c.w {
            px[(y * W + x) as usize] = CONTENT;
        }
    }
}

fn repaint(px: &mut [u32], maximized: bool) {
    paint_content(px, maximized);
    let c = content_rect_at(W, H, maximized, quarters());
    let mut fb = PaintBuffer { pixels: px, stride_words: W, width: W, height: H };
    finish_band(&mut fb, maximized, c.y, c.y + c.h);
}

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
