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

//! The largest file and the largest image the viewer takes in.
//!
//! The vfs client stops at the byte limit it is given without saying the file
//! went on, so a file past the limit used to arrive cut short and fail as a
//! damaged image; one byte past the limit tells the two apart. A decoded image
//! is copied out of the decoder's surface into the heap at four bytes a pixel
//! (and rotating it holds a second copy), so an image with too many pixels
//! would run the 192 MiB heap out and take the window down; it is refused
//! before the copy instead.

/// The largest file read as an image.
pub const MAX_IMAGE_BYTES: u32 = 16 * 1024 * 1024;

/// The byte limit an image file is read with.
pub const READ_LIMIT: u32 = MAX_IMAGE_BYTES + 1;

/// The most pixels copied out of the decoder: 80 MB, twice that while a
/// rotation holds both copies, inside the heap with the file and the gallery.
pub const MAX_PIXELS: u64 = 20_000_000;

pub const FILE_TOO_LARGE: &str = "the file is larger than 16 MiB";
pub const TOO_MANY_PIXELS: &str = "the image is larger than 20 megapixels";

/// Why a file of `len` bytes is not decoded, or `None` when it fits.
pub fn refuse_bytes(len: u64) -> Option<&'static str> {
    if len > MAX_IMAGE_BYTES as u64 {
        Some(FILE_TOO_LARGE)
    } else {
        None
    }
}

/// Why a decoded `w` by `h` image is not copied out, or `None` when it fits.
pub fn refuse_pixels(w: u32, h: u32) -> Option<&'static str> {
    if w as u64 * h as u64 > MAX_PIXELS {
        Some(TOO_MANY_PIXELS)
    } else {
        None
    }
}
