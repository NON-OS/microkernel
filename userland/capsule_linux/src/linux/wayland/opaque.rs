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

//! An XRGB8888 buffer's pixels, made opaque as they are taken. Pure, so the
//! host proofs hold it.
//!
//! The surface registry knows one format, ARGB8888, and reads the fourth
//! byte of each pixel as alpha. XRGB leaves that byte undefined, and a
//! client that leaves it 0 drew a window nobody could see.

/// Set the alpha byte of every pixel of a `width` by `height` buffer laid
/// out `stride` bytes a row in `frame`. Bytes past each row's pixels, and
/// rows past the frame's end, are left as they are.
pub fn make_opaque(frame: &mut [u8], width: u32, height: u32, stride: u32) {
    let (row_bytes, stride) = (width as usize * 4, (stride as usize).max(1));
    for row in frame.chunks_mut(stride).take(height as usize) {
        row.iter_mut().take(row_bytes).skip(3).step_by(4).for_each(|alpha| *alpha = 0xFF);
    }
}
