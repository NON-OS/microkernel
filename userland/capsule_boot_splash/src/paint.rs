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

use crate::canvas::{draw, Canvas};
use crate::look::Look;

/// The whole splash, drawn into the surface at `base` (canvas.rs).
pub(crate) fn splash(base: u64, w: u32, h: u32, stride: u32, look: &Look) {
    let spx = stride as usize / 4;
    // SAFETY: the surface at `base` is `stride * h` bytes, mapped read-write
    // by surface::setup for this drawing, and nothing else writes it.
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    let c = Canvas { fit: look.fit, chain: look.chain, dma_unconfined: look.dma_unconfined };
    draw(buf, spx, w, h, &c);
}
