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
 * The pixels a guest's window shows, kept on a page boundary. The kernel
 * registers a surface only at a page-aligned address, and a plain Vec
 * almost never starts on one: every guest window was refused with EINVAL
 * and nothing a Linux program drew ever reached the screen. The buffer
 * carries a page of slack and the frame starts at its first boundary.
 */

use alloc::vec::Vec;

const PAGE: usize = 4096;

/*
 * One registered surface's pixels. A registered buffer may not move, so a
 * buffer of another shape is given new Pixels, registered on their own,
 * and the old ones go once their surface is released (show.rs).
 */
pub struct Pixels {
    bytes: Vec<u8>,
}

impl Pixels {
    pub const fn empty() -> Pixels {
        Pixels { bytes: Vec::new() }
    }

    /// Room for a `bytes` frame, asked of the allocator rather than
    /// demanded: None when the heap has no room, and the frame is not shown
    /// while the family runs on.
    pub fn with_room(bytes: usize) -> Option<Pixels> {
        let want = bytes.checked_add(PAGE)?;
        let mut v = Vec::new();
        v.try_reserve_exact(want).ok()?;
        v.resize(want, 0);
        Some(Pixels { bytes: v })
    }

    fn offset(&self) -> usize {
        (PAGE - self.bytes.as_ptr() as usize % PAGE) % PAGE
    }

    /// The first `bytes` of the page-aligned frame; None past its end.
    pub fn frame(&mut self, bytes: usize) -> Option<&mut [u8]> {
        let at = self.offset();
        self.bytes.get_mut(at..at.checked_add(bytes)?)
    }

    /// Where the page-aligned frame starts, and how long it can be.
    pub fn span(&self) -> (u64, u64) {
        let at = self.offset().min(self.bytes.len());
        (self.bytes.as_ptr() as u64 + at as u64, (self.bytes.len() - at) as u64)
    }
}
