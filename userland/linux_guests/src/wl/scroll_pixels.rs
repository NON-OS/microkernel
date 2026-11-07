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
//! A scroll guest's buffer: a memfd sized and mapped shared, redrawn as
//! stripes that have moved on, the band at the top left alone. Unmapped and
//! closed when it is dropped, as a client lets a buffer go.

use crate::sys::{call, CLOSE, MMAP, PROT_RW};

const MUNMAP: u64 = 11;
const MEMFD_CREATE: u64 = 319;
const FTRUNCATE: u64 = 77;
const MAP_SHARED: u64 = 0x01;

const BAND_RGB: u32 = 0xFF2E_5C5C;
const STRIPES: [u32; 4] = [0xFF0A_0B0D, 0xFF66_FFFF, 0xFF1A_2B3C, 0xFF3C_6E8C];

pub struct Buffer {
    pub fd: i32,
    at: u64,
    w: usize,
    h: usize,
}

impl Buffer {
    pub fn new(w: u32, h: u32) -> Option<Buffer> {
        let size = u64::from(w).checked_mul(u64::from(h))?.checked_mul(4)?;
        let fd = call(MEMFD_CREATE, [c"scroll".as_ptr() as u64, 0, 0, 0, 0, 0]);
        if fd < 0 {
            return None;
        }
        if call(FTRUNCATE, [fd as u64, size, 0, 0, 0, 0]) != 0 {
            call(CLOSE, [fd as u64, 0, 0, 0, 0, 0]);
            return None;
        }
        let at = call(MMAP, [0, size, PROT_RW, MAP_SHARED, fd as u64, 0]);
        if at < 0 {
            call(CLOSE, [fd as u64, 0, 0, 0, 0, 0]);
            return None;
        }
        Some(Buffer { fd: fd as i32, at: at as u64, w: w as usize, h: h as usize })
    }

    fn px(&mut self) -> &mut [u32] {
        // SAFETY: `at` is this buffer's own shared mapping of w * h * 4
        // bytes, page-aligned so u32-aligned, mapped from new() until drop;
        // the &mut self borrow makes this the only reference to it.
        unsafe { core::slice::from_raw_parts_mut(self.at as *mut u32, self.w * self.h) }
    }

    /// Rows below `band` drawn as stripes `shift` rows on; the band filled.
    pub fn draw(&mut self, shift: u32, band: u32) {
        let (w, h, band) = (self.w, self.h, band as usize);
        let px = self.px();
        for y in 0..h {
            let colour = if y < band {
                BAND_RGB
            } else {
                STRIPES[((y + shift as usize) / 16) % STRIPES.len()]
            };
            px[y * w..(y + 1) * w].fill(colour);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let size = (self.w * self.h * 4) as u64;
        call(MUNMAP, [self.at, size, 0, 0, 0, 0]);
        call(CLOSE, [self.fd as u64, 0, 0, 0, 0, 0]);
    }
}
