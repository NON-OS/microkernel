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

//! The Wayland bridge's per-pixel routines as they stood before the change:
//! the reference the rows are held to.

/// `present/frame/paint.rs` `over`.
pub fn paint_over(src: u32, dst: u32) -> u32 {
    let keep = 255 - (src >> 24);
    let mix = |shift: u32| {
        let (s, d) = ((src >> shift) & 0xFF, (dst >> shift) & 0xFF);
        (s + (d * keep + 127) / 255).min(255) << shift
    };
    mix(0) | mix(8) | mix(16) | mix(24)
}

/// `subsurface/blend.rs` `on`.
pub fn on(upper: [u8; 4], lower: [u8; 4]) -> [u8; 4] {
    let through = 255 - u32::from(upper[3]);
    let mut out = [0u8; 4];
    for i in 0..4 {
        let sum = u32::from(upper[i]) + (u32::from(lower[i]) * through + 127) / 255;
        out[i] = sum.min(255) as u8;
    }
    out
}

/// `opaque.rs` `make_opaque`.
pub fn make_opaque(frame: &mut [u8], width: u32, height: u32, stride: u32) {
    let (row_bytes, stride) = (width as usize * 4, (stride as usize).max(1));
    for row in frame.chunks_mut(stride).take(height as usize) {
        row.iter_mut().take(row_bytes).skip(3).step_by(4).for_each(|alpha| *alpha = 0xFF);
    }
}

/// The pixel loop of `present/frame/paint.rs` `paint`, one row.
pub fn paint_row(dst: &mut [u32], client_row: &[u8], ground: u32) {
    for (x, px) in client_row.chunks_exact(4).take(dst.len()).enumerate() {
        let src = u32::from_le_bytes([px[0], px[1], px[2], px[3]]);
        dst[x] = if src >> 24 == 0xFF { src } else { paint_over(src, ground) };
    }
}

/// `subsurface/blend.rs` `lay`, the inner loop over one row.
pub fn lay_row(frame: &mut [u8], layer: &[u8], over: bool) {
    for (d, s) in frame.chunks_exact_mut(4).zip(layer.chunks_exact(4)) {
        let (s, below) = ([s[0], s[1], s[2], s[3]], [d[0], d[1], d[2], d[3]]);
        d.copy_from_slice(&if over { on(s, below) } else { on(below, s) });
    }
}

/// `present/resample.rs` `resample`.
pub fn resample(
    src: &[u8],
    (sw, sh, stride): (u32, u32, u32),
    dst: &mut [u8],
    (dw, dh): (u32, u32),
) -> bool {
    let (sw, sh, stride, dw, dh) =
        (sw as usize, sh as usize, stride as usize, dw as usize, dh as usize);
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 || stride < sw * 4 {
        return false;
    }
    if src.len() < (sh - 1) * stride + sw * 4 || dst.len() < dw * dh * 4 {
        return false;
    }
    for (y, out) in dst.chunks_exact_mut(dw * 4).take(dh).enumerate() {
        let row = ((2 * y + 1) * sh / (2 * dh)).min(sh - 1) * stride;
        for (x, px) in out.chunks_exact_mut(4).enumerate() {
            let at = row + ((2 * x + 1) * sw / (2 * dw)).min(sw - 1) * 4;
            px.copy_from_slice(&src[at..at + 4]);
        }
    }
    true
}

/// `damage_lay.rs` `lay`, one row, with the blend `damage_part.rs` gave it.
pub fn lay_box_row(dst: &mut [u8], src: &[u8], ground: u32) {
    let blend = |px: u32| if px >> 24 == 0xFF { px } else { paint_over(px, ground) };
    for (d, s) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
        d.copy_from_slice(&blend(u32::from_le_bytes([s[0], s[1], s[2], s[3]])).to_le_bytes());
    }
}

/// The compositor's `sw_blitter/row.rs` `composite_row`, on words.
pub fn composite_row(src: &[u32], dst: &mut [u32]) {
    let w = core::cmp::min(src.len(), dst.len());
    let mut col = 0usize;
    while col < w {
        let a = src[col] >> 24;
        if a == 0xFF {
            let start = col;
            while col < w && src[col] >> 24 == 0xFF {
                col += 1;
            }
            dst[start..col].copy_from_slice(&src[start..col]);
        } else if a == 0 {
            col += 1;
        } else {
            dst[col] = blend(src[col], dst[col], a);
            col += 1;
        }
    }
}

/// The compositor's `sw_blitter/row.rs` `blend`.
pub fn blend(src: u32, dst: u32, a: u32) -> u32 {
    let ia = 255 - a;
    let rb = (src & 0x00FF_00FF) * a + (dst & 0x00FF_00FF) * ia;
    let g = ((src >> 8) & 0xFF) * a + ((dst >> 8) & 0xFF) * ia;
    let rb = ((rb + 0x0001_0001 + ((rb >> 8) & 0x00FF_00FF)) >> 8) & 0x00FF_00FF;
    let g = (g + 1 + (g >> 8)) >> 8;
    0xFF00_0000 | rb | (g << 8)
}
