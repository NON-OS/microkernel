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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::css::ObjectFit;

use super::super::store::Decoded;
use super::draw::draw;
use super::place::place;

/* Scale-blit `img` into the box at (x, y, box_w, box_h) honouring object-fit:
 * contain letterboxes the whole image, cover fills the box and crops the
 * overflow, fill stretches to the box. Bilinear sampling keeps scaled images
 * smooth; alpha is composited so transparent PNGs read correctly. */
pub fn blit_into(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [u32; 4],
    fit: ObjectFit,
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    let [x, y, w, h] = dest.map(|v| v.min(i32::MAX as u32) as i32);
    blit_rect(fb, img, [x, y, w, h], fit, alpha, clip);
}

/* The same with a signed origin, for a box that starts above or left of the
 * framebuffer: only the part inside the clip and the framebuffer is drawn,
 * sampled exactly as it would be if the whole box were on screen. */
pub fn blit_rect(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [i32; 4],
    fit: ObjectFit,
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    if alpha == 0 {
        return;
    }
    if let Some((dest, src)) = place(fit, dest, img.w, img.h) {
        draw(fb, img, dest, src, alpha, clip);
    }
}
