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

//! A 0xff0000ff framebuffer with one gradient painted into it.
use nonos_app_skeleton::PaintBuffer;
use std::vec::Vec;

use crate::shim::paint_gradient;

pub fn paint(w: u32, h: u32, src: &str, rect: [i32; 4], clip: [i32; 4]) -> Vec<u32> {
    let mut px = std::vec![0xff00_00ffu32; (w * h) as usize];
    assert!(paint_gradient(
        &mut PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h },
        src,
        rect,
        clip
    ));
    px
}
