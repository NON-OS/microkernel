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

//! A rectangle of the mode, copied between two spans of it: from the surface
//! the compositor draws in to the framebuffer the adapter scans out. Its
//! corners come off the wire, so where it lies is counted checked and held
//! to both spans before a pixel moves.

use core::ptr::{read_volatile, write_volatile};

use super::span::Span;
use crate::constants::BYTES_PER_PIXEL;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// The index of `r`'s first pixel in a mode `width` pixels a line, or
/// `None` when `r` is empty, runs past the right edge, or ends past `pixels`.
/// With the right edge held, ending within `width` x `height` pixels is
/// ending above the bottom edge.
fn first(r: Rect, width: u16, pixels: u64) -> Option<u64> {
    if r.width == 0 || r.height == 0 || r.x.checked_add(r.width)? > width as u32 {
        return None;
    }
    let line = width as u64;
    let start = (r.y as u64).checked_mul(line)?.checked_add(r.x as u64)?;
    let end = (r.height as u64 - 1).checked_mul(line)?.checked_add(start)?;
    (end.checked_add(r.width as u64)? <= pixels).then_some(start)
}

/// Where `r` starts in `fb`, `width` pixels a line, in bytes; `None` when it
/// does not lie within the mode.
pub fn place(fb: &Span, width: u16, r: Rect) -> Option<u64> {
    first(r, width, fb.pixels).map(|p| p * BYTES_PER_PIXEL as u64)
}

/// Copy `r` from `from` to the same place in `to`, both `width` pixels a
/// line. False, with nothing written, when `r` lies outside either.
pub fn copy(from: &Span, to: &Span, width: u16, r: Rect) -> bool {
    let Some(start) = first(r, width, from.pixels.min(to.pixels)) else {
        return false;
    };
    let (src, dst) = (from.va as *const u32, to.va as *mut u32);
    for row in 0..r.height as u64 {
        // At most the last line's start, which `first` counted checked.
        let at = start + row * width as u64;
        for i in at..at + r.width as u64 {
            // SAFETY: pixel `i` lies below the end `first` held within both
            // spans, and `span` made each only for pixels its mapping holds.
            // The compositor may draw while this reads: a torn pixel is the
            // worst of it, as every bit pattern is a valid u32.
            unsafe { write_volatile(dst.add(i as usize), read_volatile(src.add(i as usize))) };
        }
    }
    true
}
