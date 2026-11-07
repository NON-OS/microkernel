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
//! Showing a shadow on a surface: its rectangles copied into the surface's
//! DMA (each row checked inside both sides by `copy_rects`), each sent to
//! the host's copy with TRANSFER_TO_HOST_2D at its offset in the backing,
//! and then flushed to the screen. A command the host refuses or drops
//! fails the whole, after the copy.

use core::sync::atomic::{fence, Ordering};

use nonos_gfx_serve::{copy_rects, Shadow};
use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::{Errno, Rect};

use super::gpu::Gpu;
use super::surface::Surface;
use crate::device::cmd::batch_2d;
use crate::device::cmd::transfer_to_host_2d::Rect as Area;

pub fn area(r: &Rect) -> Area {
    Area { x: r.x, y: r.y, width: r.width, height: r.height }
}

impl Gpu {
    /// `rects` of `shadow` into `s` and on to the host's copy of it.
    pub fn load(&self, s: &Surface, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        let stride = s.width as usize * 4;
        // SAFETY: the surface's DMA is mapped, `len` bytes, for as long as
        // the surface is this driver's; copy_rects keeps rows inside it.
        unsafe {
            let dst = s.map.user_va as *mut u8;
            copy_rects(shadow.bytes, shadow.stride as usize, dst, s.bytes(), stride, rects, false);
        }
        self.send(s, rects)
    }

    /// `rects` of `s`'s backing sent to the host's copy as they stand.
    pub fn send(&self, s: &Surface, rects: &[Rect]) -> Result<(), Errno> {
        // The stores land before the host is told to read them.
        fence(Ordering::Release);
        let mut areas = rects.iter().map(area);
        batch_2d::transfer_rects(&self.q, &self.fences, (s.id, s.width), &mut areas)
            .map_err(|_| ETIMEDOUT)
    }

    /// `rects` of `s` flushed to the screens showing it.
    pub fn flush_rects(&self, s: &Surface, rects: &[Rect]) -> Result<(), Errno> {
        let mut areas = rects.iter().map(area);
        batch_2d::flush_rects(&self.q, &self.fences, s.id, &mut areas).map_err(|_| ETIMEDOUT)
    }
}
