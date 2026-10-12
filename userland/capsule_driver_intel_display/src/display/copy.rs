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

//! Rectangles of the shadow into one scanout surface, row by row at the
//! surface's stride. Every rectangle is checked whole, against the shadow
//! and the surface both, before its first row is written.

use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{Errno, Rect};

use super::frames::Frames;

/// `size` and `stride` are the surface's.
pub fn copy<F: Frames>(
    f: &mut F,
    frame: usize,
    size: (u32, u32),
    stride: u32,
    rects: &[Rect],
    shadow: &Shadow,
) -> Result<(), Errno> {
    let w = size.0.min(shadow.width) as u64;
    let h = size.1.min(shadow.height) as u64;
    for r in rects {
        let inside = r.x as u64 + r.width as u64 <= w && r.y as u64 + r.height as u64 <= h;
        if !inside {
            return Err(EINVAL);
        }
    }
    for r in rects {
        let (x, len) = (r.x as usize * 4, r.width as usize * 4);
        for y in r.y as usize..(r.y + r.height) as usize {
            let from = y * shadow.stride as usize + x;
            let row = shadow.bytes.get(from..from + len).ok_or(EINVAL)?;
            if !f.put(frame, y * stride as usize + x, row) {
                return Err(EINVAL);
            }
        }
    }
    Ok(())
}
