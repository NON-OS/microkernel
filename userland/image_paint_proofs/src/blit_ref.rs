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

//! The blit as it was before clipping moved out of the pixel loop: every
//! destination pixel visited, its source position divided out, the bilinear
//! blend done a channel at a time. The new blit must match it bit for bit.
use nonos_app_skeleton::PaintBuffer;

use crate::browser::image::Decoded;

pub fn draw(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [u32; 4],
    src: [u32; 4],
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    let ([dx, dy, dw, dh], [sx0, sy0, sw, sh]) = (dest, src);
    for j in 0..dh {
        let gy = (sy0 as u64 * 256 + j as u64 * sh as u64 * 256 / dh as u64) as u32;
        for i in 0..dw {
            let (px, py) = (dx + i, dy + j);
            if let Some([cx0, cy0, cx1, cy1]) = clip {
                if (px as i32) < cx0 || px as i32 >= cx1 || (py as i32) < cy0 || py as i32 >= cy1 {
                    continue;
                }
            }
            let gx = (sx0 as u64 * 256 + i as u64 * sw as u64 * 256 / dw as u64) as u32;
            let (w, h) = (img.w, img.h);
            let (x0, y0) = ((gx >> 8).min(w - 1), (gy >> 8).min(h - 1));
            let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
            let (fx, fy) = (gx & 0xff, gy & 0xff);
            let t = |xx: u32, yy: u32| img.px[(yy * w + xx) as usize];
            let c = [t(x0, y0), t(x1, y0), t(x0, y1), t(x1, y1)];
            let chan = |s: u32| {
                let v = c.map(|p| (p >> s) & 0xff);
                let top = v[0] * (256 - fx) + v[1] * fx;
                let bot = v[2] * (256 - fx) + v[3] * fx;
                ((top * (256 - fy) + bot * fy) >> 16) & 0xff
            };
            let argb = (chan(24) << 24) | (chan(16) << 16) | (chan(8) << 8) | chan(0);
            put(fb, px, py, argb, alpha);
        }
    }
}

fn put(fb: &mut PaintBuffer, x: u32, y: u32, argb: u32, alpha: u8) {
    let idx = y as usize * fb.stride_words as usize + x as usize;
    if x >= fb.width || y >= fb.height || idx >= fb.pixels.len() {
        return;
    }
    let a = (((argb >> 24) & 0xff) * alpha as u32) / 255;
    if a == 0 {
        return;
    }
    let dst = fb.pixels[idx];
    let mix = |s: u32, d: u32| (s * a + d * (255 - a)) / 255;
    let [r, g, b] = [16, 8, 0].map(|s| mix((argb >> s) & 0xff, (dst >> s) & 0xff));
    fb.pixels[idx] = 0xff00_0000 | (r << 16) | (g << 8) | b;
}
