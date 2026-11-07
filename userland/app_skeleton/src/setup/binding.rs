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

use crate::swap::Swap;

/// Buffers a window draws in: two, a project decision (triple only where
/// a client asks, which no native app does).
pub const BUFFERS: usize = 2;

/// One of a window's buffers: its surface id (this process's), its handle
/// (the compositor's, read only) and where it is mapped here.
#[derive(Clone, Copy)]
pub struct Buffer {
    pub sid: u64,
    pub handle: u64,
    pub va: u64,
}

pub struct WindowBinding {
    pub bufs: [Buffer; BUFFERS],
    pub swap: Swap,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub stride_words: u32,
    /// Each buffer's length.
    pub byte_len: u64,
}

impl WindowBinding {
    /// The handle the compositor's layer shows: what every op naming the
    /// window's layer by its surface sends.
    pub fn surface_handle(&self) -> u64 {
        self.swap.shown_handle()
    }

    /// Buffer `i`'s pixels. Only a buffer `swap` handed out to draw in is
    /// written: the compositor reads the others.
    pub fn pixels(&mut self, i: usize) -> &mut [u32] {
        let words = (self.byte_len / 4) as usize;
        let va = self.bufs[i % BUFFERS].va;
        // SAFETY: each buffer is this process's mapping of `byte_len` bytes
        // for as long as the binding holds it (setup/buffers.rs maps it,
        // teardown unmaps it), and nothing else here writes it.
        unsafe { core::slice::from_raw_parts_mut(va as *mut u32, words) }
    }

    /// Buffer `i`'s pixels, to read.
    pub fn read(&self, i: usize) -> &[u32] {
        let words = (self.byte_len / 4) as usize;
        let va = self.bufs[i % BUFFERS].va;
        // SAFETY: the mapping is the one `pixels` hands out, `byte_len`
        // bytes for as long as the binding holds it. Only `pixels` writes
        // it here and it needs the binding mutably, so nothing here writes
        // it while this borrow lives; the compositor maps it read only.
        unsafe { core::slice::from_raw_parts(va as *const u32, words) }
    }
}
