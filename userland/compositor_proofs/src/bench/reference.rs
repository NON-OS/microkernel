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

//! The blitter's routines as they stood before the change, the reference the
//! shipping ones are held to and timed against. Test crate only.

use crate::damage::Rect;
use crate::sw_blitter::Surface;

/// `sw_blitter/upscale.rs` `upscale`.
pub fn upscale(canvas: Surface, screen: Surface, rect: Rect, scale: u32) -> Option<Rect> {
    if scale == 0 || rect.x >= canvas.width || rect.y >= canvas.height {
        return None;
    }
    let w = rect.width.min(canvas.width - rect.x);
    let h = rect.height.min(canvas.height - rect.y);
    let (sx, sy) = (rect.x.checked_mul(scale)?, rect.y.checked_mul(scale)?);
    if w == 0 || h == 0 || sx >= screen.width || sy >= screen.height {
        return None;
    }
    let sw = w.checked_mul(scale)?.min(screen.width - sx);
    let sh = h.checked_mul(scale)?.min(screen.height - sy);
    for row in 0..sh {
        let src_va = canvas.row_start(rect.y + row / scale, rect.x, w)?;
        let dst_va = screen.row_start(sy + row, sx, sw)?;
        // SAFETY: as the original: both rows bounds checked, distinct buffers.
        let src: &[u32] = unsafe { core::slice::from_raw_parts(src_va as *const u32, w as usize) };
        let dst: &mut [u32] =
            unsafe { core::slice::from_raw_parts_mut(dst_va as *mut u32, sw as usize) };
        for (i, px) in dst.iter_mut().enumerate() {
            *px = src[i / scale as usize];
        }
    }
    Some(Rect { x: sx, y: sy, width: sw, height: sh })
}

/// `sw_blitter/stretch.rs` `stretch`.
pub fn stretch(canvas: Surface, screen: Surface, rect: Rect, q: u32) -> Option<Rect> {
    if q < 4 || rect.x >= canvas.width || rect.y >= canvas.height {
        return None;
    }
    let q = q as u64;
    let x1 = (rect.x as u64 + rect.width as u64).min(canvas.width as u64);
    let y1 = (rect.y as u64 + rect.height as u64).min(canvas.height as u64);
    let first = |c: u64| (c * q + 1) / 4;
    let (sx0, sy0) = (first(rect.x as u64), first(rect.y as u64));
    let sx1 = first(x1).min(screen.width as u64);
    let sy1 = first(y1).min(screen.height as u64);
    if sx0 >= sx1 || sy0 >= sy1 {
        return None;
    }
    let src = |s: u64| ((4 * s + 2) / q) as u32;
    let n = (sx1 - sx0) as u32;
    for sy in sy0..sy1 {
        let dst_va = screen.row_start(sy as u32, sx0 as u32, n)?;
        let row = canvas.row_start(src(sy), 0, canvas.width)?;
        // SAFETY: as the original: both rows bounds checked, distinct buffers.
        let (dst, line) = unsafe {
            let d = core::slice::from_raw_parts_mut(dst_va as *mut u32, n as usize);
            (d, core::slice::from_raw_parts(row as *const u32, canvas.width as usize))
        };
        for (i, px) in dst.iter_mut().enumerate() {
            *px = line[(src(sx0 + i as u64) as usize).min(line.len() - 1)];
        }
    }
    Some(Rect { x: sx0 as u32, y: sy0 as u32, width: n, height: (sy1 - sy0) as u32 })
}

/// `sw_blitter/row.rs` `composite_row` with its `blend`.
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
            let (s, d, ia) = (src[col], dst[col], 255 - a);
            let rb = (s & 0x00FF_00FF) * a + (d & 0x00FF_00FF) * ia;
            let g = ((s >> 8) & 0xFF) * a + ((d >> 8) & 0xFF) * ia;
            let rb = ((rb + 0x0001_0001 + ((rb >> 8) & 0x00FF_00FF)) >> 8) & 0x00FF_00FF;
            let g = (g + 1 + (g >> 8)) >> 8;
            dst[col] = 0xFF00_0000 | rb | (g << 8);
            col += 1;
        }
    }
}

/// A surface over `px`, `w` pixels a row.
pub fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    Surface {
        base_va: px.as_mut_ptr() as u64,
        stride: w * 4,
        width: w,
        height: h,
        byte_len: px.len() as u64 * 4,
    }
}
