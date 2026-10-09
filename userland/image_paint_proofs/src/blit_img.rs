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

//! A test image with translucent and transparent texels, and a framebuffer
//! filled with a pattern for the blit to land on.
use nonos_app_skeleton::PaintBuffer;
use std::vec::Vec;

use crate::browser::image::Decoded;

pub const FW: u32 = 97;
pub const FH: u32 = 61;

pub fn image(w: u32, h: u32) -> Decoded {
    let px = (0..w * h)
        .map(|i| i.wrapping_mul(0x9E37_79B1) | if i % 5 == 0 { 0 } else { 0x4000_0000 })
        .collect();
    Decoded { w, h, px }
}

pub fn frame(f: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let mut px: Vec<u32> =
        (0..FW * FH).map(|i| 0xff00_0000 | i.wrapping_mul(2_654_435_761) >> 8).collect();
    f(&mut PaintBuffer { pixels: &mut px, stride_words: FW, width: FW, height: FH });
    px
}
