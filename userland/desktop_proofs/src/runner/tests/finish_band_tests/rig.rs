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

//! A window's content painted and partly repainted.

use super::super::super::chrome::quarters;
use super::super::super::finish_band::finish_band;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::content_rect_at;

pub(super) const W: u32 = 160;

pub(super) const H: u32 = 120;

pub(super) const FRAME: u32 = 0xFF2A_2E36;

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

pub(super) fn repaint(px: &mut [u32], maximized: bool) {
    paint_content(px, maximized);
    let c = content_rect_at(W, H, maximized, quarters());
    let mut fb = PaintBuffer { pixels: px, stride_words: W, width: W, height: H };
    finish_band(&mut fb, maximized, c.y, c.y + c.h);
}
