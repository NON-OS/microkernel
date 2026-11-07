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

//! The compositor's layer rule: straight (not premultiplied) alpha, the
//! result always opaque. An opaque source pixel is copied, a clear one
//! leaves the destination as it was, any other is mixed
//! `(x + 1 + (x >> 8)) >> 8` with `x = s * a + d * (255 - a)` per colour.

use crate::level::{level, Level};
use crate::scalar::load;

/// `src` laid on `dst`, whole pixels, as many as both hold.
pub fn composite_row(dst: &mut [u8], src: &[u8]) {
    let n = dst.len().min(src.len()) / 4 * 4;
    let (dst, src) = (&mut dst[..n], &src[..n]);
    // SAFETY: the level says the processor runs these instructions; both
    // rows hold `k` bytes.
    let done = match level() {
        #[cfg(target_arch = "x86_64")]
        Level::Avx2 => {
            let k = n / 32 * 32;
            unsafe { crate::avx2::composite(dst.as_mut_ptr(), src.as_ptr(), k) };
            k
        }
        #[cfg(target_arch = "x86_64")]
        Level::Sse2 => {
            let k = n / 16 * 16;
            unsafe { crate::sse2::composite(dst.as_mut_ptr(), src.as_ptr(), k) };
            k
        }
        _ => 0,
    };
    scalar(&mut dst[done..], &src[done..]);
}

/// The plain path, and the tail of the vector ones: runs of opaque pixels
/// copied whole, clear ones skipped, the rest mixed.
pub fn scalar(dst: &mut [u8], src: &[u8]) {
    let w = dst.len().min(src.len()) / 4;
    let mut col = 0;
    while col < w {
        let s = load(&src[col * 4..]);
        let a = s >> 24;
        if a == 0xFF {
            let start = col;
            while col < w && load(&src[col * 4..]) >> 24 == 0xFF {
                col += 1;
            }
            dst[start * 4..col * 4].copy_from_slice(&src[start * 4..col * 4]);
        } else {
            if a != 0 {
                let d = load(&dst[col * 4..]);
                dst[col * 4..col * 4 + 4].copy_from_slice(&mix(s, d, a).to_le_bytes());
            }
            col += 1;
        }
    }
}

/// One pixel of `src` at alpha `a` (1 to 254) over `dst`.
#[inline]
pub fn mix(src: u32, dst: u32, a: u32) -> u32 {
    let ia = 255 - a;
    let rb = (src & 0x00FF_00FF) * a + (dst & 0x00FF_00FF) * ia;
    let g = ((src >> 8) & 0xFF) * a + ((dst >> 8) & 0xFF) * ia;
    let rb = ((rb + 0x0001_0001 + ((rb >> 8) & 0x00FF_00FF)) >> 8) & 0x00FF_00FF;
    let g = (g + 1 + (g >> 8)) >> 8;
    0xFF00_0000 | rb | (g << 8)
}
