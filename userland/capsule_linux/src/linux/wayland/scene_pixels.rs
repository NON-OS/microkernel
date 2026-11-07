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

/*
 * The pixels a guest's window shows, in a mapping of their own. The kernel
 * registers a surface only at a page-aligned address, which a mapping
 * always is. A registered buffer may not move, so a buffer of another shape
 * is given new Pixels, registered on their own, and the old ones go once
 * their surface is released (show.rs).
 *
 * The pixels were a Vec in the family's heap. Released while the compositor
 * still viewed it, a surface keeps its slot and its frames until the
 * compositor lets go, but the Vec went back to the heap at once, and the
 * next window buffer was given those same pages: the kernel refuses to
 * register frames another slot holds, so the window kept its old picture.
 * Telegram restored from maximised stayed drawn at its maximised size,
 * `surface 1045x795 refused, rc -1`. Unmapped instead, the frames of a
 * surface still viewed become an orphan the kernel frees once the last
 * view goes, and the next buffer is new pages.
 */

use core::slice;

use super::scene_pixels_map::{map, unmap};

const PAGE: usize = 4096;

/// One registered surface's pixels: `len` bytes mapped at `at`, or none.
pub struct Pixels {
    at: usize,
    len: usize,
}

impl Pixels {
    pub const fn empty() -> Pixels {
        Pixels { at: 0, len: 0 }
    }

    /// Room for a `bytes` frame, in whole pages asked of the kernel: None
    /// when it has none to give, and the frame is not shown while the
    /// family runs on.
    pub fn with_room(bytes: usize) -> Option<Pixels> {
        let len = bytes.max(1).checked_add(PAGE - 1)? & !(PAGE - 1);
        let at = map(len)?;
        Some(Pixels { at, len })
    }

    /// The first `bytes` of the frame; None past its end.
    pub fn frame(&mut self, bytes: usize) -> Option<&mut [u8]> {
        if bytes > self.len {
            return None;
        }
        if self.len == 0 {
            return Some(&mut []);
        }
        // SAFETY: [at, at + len) is this value's own mapping, readable and
        // writable until it is dropped, and `bytes` is within it; `&mut self`
        // is the only borrow of it in this process.
        Some(unsafe { slice::from_raw_parts_mut(self.at as *mut u8, bytes) })
    }

    /// The same, to read.
    pub fn frame_ref(&self, bytes: usize) -> Option<&[u8]> {
        if bytes > self.len {
            return None;
        }
        if self.len == 0 {
            return Some(&[]);
        }
        // SAFETY: as in `frame`; `&self` holds off any writer here.
        Some(unsafe { slice::from_raw_parts(self.at as *const u8, bytes) })
    }

    /// Where the frame starts, and how long it can be.
    pub fn span(&self) -> (u64, u64) {
        (self.at as u64, self.len as u64)
    }
}

impl Default for Pixels {
    fn default() -> Pixels {
        Pixels::empty()
    }
}

/// The mapping given back whole. A surface not released is never dropped
/// here: its pixels are forgotten instead (scene_buffers.rs release).
impl Drop for Pixels {
    fn drop(&mut self) {
        if self.len != 0 {
            unmap(self.at, self.len);
        }
    }
}
