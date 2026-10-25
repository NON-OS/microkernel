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

use super::buffer::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    /* Row by row, each its width: a sub-buffer's slice runs on through the
    stride between its rows, which is the parent's (a window's frame and
    shadow beside its content), and a whole-slice fill painted over it. */
    pub fn clear(&mut self, argb: u32) {
        let (w, stride) = (self.width as usize, self.stride_words as usize);
        for row in 0..self.height as usize {
            let at = row * stride;
            let end = (at + w).min(self.pixels.len());
            if at < end {
                self.pixels[at..end].fill(argb);
            }
        }
    }
}
