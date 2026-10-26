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

//! A vision tier's image: `qwen vl-2b QUESTION < FILE` hands FILE to the
//! tier, ahead of the question, as one frame on its stdin (qwenchat's
//! qwenvision_frame.h): "NXIMG01\n", the length as a little-endian u32, then
//! the bytes. A vision tier is always sent a frame, empty with no image, so
//! qwenchat knows where the question starts. Any other tier's line is
//! untouched: its question is the line as typed, `<` and all.

use alloc::vec::Vec;

/// The tiers that are shown an image.
pub const VISION: &[&[u8]] = &[b"vl-2b"];

/// The frame's head.
pub const MAGIC: &[u8; 8] = b"NXIMG01\n";

#[path = "image/split.rs"]
mod split;
pub use split::split;

/// Whether `tier` is shown an image.
pub fn is_vision(tier: &[u8]) -> bool {
    VISION.contains(&tier)
}

/// The most an image may be so its frame and the question fit what the
/// terminal holds for a program (`queue`, less one byte for the line end).
pub fn image_max(queue: usize, question: &[u8]) -> usize {
    queue.saturating_sub(MAGIC.len() + 4 + question.len() + 1)
}

/// The frame for `image` (empty: no image).
pub fn frame(image: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(MAGIC.len() + 4 + image.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(image.len() as u32).to_le_bytes());
    out.extend_from_slice(image);
    out
}
