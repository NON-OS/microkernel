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

use super::scene::Scene;

const PAGE: usize = 4096;

impl Scene {
    fn frame_offset(&self) -> usize {
        (PAGE - self.pixels.as_ptr() as usize % PAGE) % PAGE
    }

    /// Room for a `bytes` frame. False when the buffer is too small and
    /// already registered, since a registered buffer may not move.
    pub fn frame_room(&mut self, bytes: usize) -> bool {
        if self.pixels.len().saturating_sub(self.frame_offset()) >= bytes {
            return true;
        }
        if self.out.is_some() {
            return false;
        }
        self.pixels.resize(bytes + PAGE, 0);
        true
    }

    /// The first `bytes` of the page-aligned frame.
    pub fn frame(&mut self, bytes: usize) -> &mut [u8] {
        let at = self.frame_offset();
        &mut self.pixels[at..at + bytes]
    }

    /// Where the page-aligned frame starts, and how long it can be.
    pub fn frame_span(&self) -> (u64, u64) {
        let at = self.frame_offset();
        (self.pixels.as_ptr() as u64 + at as u64, (self.pixels.len() - at) as u64)
    }
}
