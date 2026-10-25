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

//! Firmware's picture into both scanout surfaces before the plane moves to
//! them, so taking the output over shows no black frame. It is read once
//! through a write-combining window of the aperture at the plane's live
//! graphics address (GMADR maps the GGTT's low part: PRM GMADR, as known
//! to the author). A picture outside the aperture, or a window the broker
//! refuses, leaves the surfaces black until the compositor's first FLUSH.

use alloc::vec;

use nonos_intel_display::display::{red_first, swap_red_blue, Frames};
use nonos_intel_display::engine::plane_read::PlaneState;
use nonos_libc::mk_mmio_unmap;

use super::maps;
use crate::frames::DmaFrames;
use crate::handles::BrokerHandles;

/// Whether the picture was copied.
pub fn copy_picture(
    h: &BrokerHandles,
    plane: &PlaneState,
    frames: &mut DmaFrames,
    stride: u32,
    aperture: u64,
) -> bool {
    let (off, rows) = (plane.live as u64, plane.height as usize);
    let len = (plane.stride as u64 * rows as u64).next_multiple_of(4096);
    if off.saturating_add(len) > aperture {
        return false;
    }
    let Ok(win) = maps::aperture(h, off, len) else { return false };
    let mut row = vec![0u8; plane.width as usize * 4];
    let swap = red_first(plane);
    for y in 0..rows {
        let from = win.user_va + (y * plane.stride as usize) as u64;
        // SAFETY: the window maps `len` bytes, every row of the plane.
        unsafe { core::ptr::copy_nonoverlapping(from as *const u8, row.as_mut_ptr(), row.len()) };
        if swap {
            swap_red_blue(&mut row);
        }
        let at = y * stride as usize;
        let _ = frames.put(0, at, &row) && frames.put(1, at, &row);
    }
    let _ = mk_mmio_unmap(win.grant_id);
    true
}
