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

//! The surface the compositor draws in: ordinary memory, mapped here,
//! registered with the kernel and shared under the handle get_primary hands
//! the compositor. A flush copies from it into the framebuffer, so the
//! framebuffer mapping, which is device memory, stays this driver's alone.

use nonos_libc::{
    mk_surface_register, mk_surface_share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

use super::memory::{map, Memory};
use crate::constants::{BYTES_PER_PIXEL, MAX_SURFACE_BYTES};
use crate::dispi::{span, Span};
use crate::error::{BgaError, BgaResult};

pub struct Surface {
    pub pixels: Span,
    pub handle: u64,
    _memory: Memory,
}

/// A `width` x `height` surface, one 32-bit pixel each, lines packed.
pub fn make(width: u16, height: u16) -> BgaResult<Surface> {
    // Two 16-bit sides of four bytes a pixel stay below 2^34: no overflow.
    let stride = width as u32 * BYTES_PER_PIXEL;
    let byte_len = stride as u64 * height as u64;
    if byte_len == 0 || byte_len > MAX_SURFACE_BYTES {
        return Err(BgaError::SurfaceRefused);
    }
    let memory = map(byte_len as usize)?;
    let pixels = span(memory.va, byte_len, width, height)?;
    let (width, height, format) = (width as u32, height as u32, SURFACE_FORMAT_ARGB8888);
    let base_va = memory.va;
    let desc = SurfaceDescriptor { width, height, stride, format, byte_len, base_va, flags: 0 };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return Err(BgaError::SurfaceRefused);
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        return Err(BgaError::SurfaceRefused);
    }
    Ok(Surface { pixels, handle: handle as u64, _memory: memory })
}
