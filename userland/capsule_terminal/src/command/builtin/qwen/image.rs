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

/// Whether `tier` is shown an image.
pub fn is_vision(tier: &[u8]) -> bool {
    VISION.contains(&tier)
}

/// For a vision tier, a question that ends ` < FILE`, FILE one word naming a
/// PNG or JPEG by its ending, as the question before it and FILE; otherwise
/// the question whole and no file, so "is 2 < 3?" is still a question.
pub fn split<'a>(tier: &[u8], question: &'a [u8]) -> (&'a [u8], Option<&'a [u8]>) {
    if !is_vision(tier) {
        return (question, None);
    }
    let Some(lt) = question.iter().rposition(|&b| b == b'<') else { return (question, None) };
    let path = trim(&question[lt + 1..]);
    let before = &question[..lt];
    let spaced = lt == 0 || before.last().is_some_and(|&b| b == b' ' || b == b'\t');
    if path.is_empty() || path.iter().any(|&b| b == b' ' || b == b'\t') || !spaced || !image_name(path) {
        return (question, None);
    }
    (trim(before), Some(path))
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

/// Whether `path` ends .png, .jpg or .jpeg, in any case.
fn image_name(path: &[u8]) -> bool {
    let low: Vec<u8> = path.iter().map(u8::to_ascii_lowercase).collect();
    [&b".png"[..], b".jpg", b".jpeg"].iter().any(|e| low.len() > e.len() && low.ends_with(e))
}

fn trim(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| c != b' ' && c != b'\t').unwrap_or(b.len());
    let end = b.iter().rposition(|&c| c != b' ' && c != b'\t').map_or(start, |i| i + 1);
    &b[start..end]
}
