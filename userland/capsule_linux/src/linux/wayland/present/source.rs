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

//! Where a committed buffer's pixels are, and how they are shaped.

use crate::linux::guest::memfd_spans::resolve;
use crate::linux::guest::MemfdSpan;

use super::super::frame_len::frame_len;
use super::super::reshape::Shape;
use super::super::scene::Scene;
use super::super::shm_fit::read_len;

const NO_POOL: &[u8] = b"[WAYLAND] commit of a buffer with no pool behind it, not shown\n";
const NO_FRAME: &[u8] =
    b"[WAYLAND] commit of a buffer empty, outside its pool or larger than a frame, not shown\n";
const NO_FIT: &[u8] =
    b"[WAYLAND] buffer whose stride, width or height does not fit its pool now, not shown\n";
const UNMAPPED: &[u8] =
    b"[WAYLAND] buffer whose memfd is not mapped under it now, not shown\n";

#[derive(Clone, Copy)]
pub struct Source {
    /// Where the pixels start in the guest.
    pub at: u64,
    /// The shape they are shown at, and how many bytes that is.
    pub shape: Shape,
    pub bytes: usize,
    /// XRGB: its alpha bytes are set as it is taken (opaque.rs).
    pub opaque: bool,
    /// The buffer's own shape and bytes (`frame_len`), when it is shown at
    /// another size (scaled.rs).
    pub raw: Option<(Shape, usize)>,
    /// How many bytes from `at` are the guest's to read: `bytes`, but for a
    /// window geometry's box, whose last row ends at its own edge (crop.rs).
    pub read: usize,
    /// The box's corner in the buffer's pixels, and in the surface's
    /// logical ones: (0, 0) for a buffer shown whole.
    pub cut: (u32, u32),
    pub origin: (i32, i32),
}

/// The pixels of `buffer`, or what to say instead. Asked at every read, so
/// what is read is the buffer's own shape, inside its pool as the pool is
/// now, from where the guest has the pool's memfd mapped now: a buffer whose
/// rows would not fit (a stride under its width, a pool shrunk under it) or
/// whose memfd went is not shown, rather than read at another shape.
pub fn source(scene: &Scene, spans: &[MemfdSpan], buffer: u32) -> Result<Source, &'static [u8]> {
    let b = scene.buffers.iter().find(|b| b.id == buffer).ok_or(NO_POOL)?;
    let pool = scene.pools.iter().find(|p| p.id == b.pool).ok_or(NO_POOL)?;
    read_len(b.offset, b.width, b.height, b.stride, pool.size).ok_or(NO_FIT)?;
    let bytes = frame_len(b.offset, b.stride, b.height, pool.size).ok_or(NO_FRAME)?;
    let at = resolve(spans, pool.memfd, b.offset, bytes as u64).ok_or(UNMAPPED)?;
    let shape = (b.width, b.height, b.stride);
    let (raw, read, cut, origin) = (None, bytes, (0, 0), (0, 0));
    Ok(Source { at, shape, bytes, opaque: b.opaque, raw, read, cut, origin })
}

/// `src` cut to box `b` of its buffer, the window geometry at logical
/// corner `origin` (crop.rs): its rows at the buffer's stride, read up to
/// the box's last pixel.
pub fn cropped(src: Source, b: super::crop::Area, origin: (i32, i32)) -> Source {
    let stride = src.shape.2;
    let (start, read) = super::crop::span(b, stride);
    let bytes = b.3 as usize * stride as usize;
    let at = src.at.saturating_add(start);
    Source { at, shape: (b.2, b.3, stride), bytes, read, cut: (b.0, b.1), origin, ..src }
}

/// Read `src` out of the guest into `frame`, opaque when it is XRGB.
pub fn take(pid: u32, src: &Source, frame: &mut [u8]) -> bool {
    if let Some(raw) = src.raw {
        return super::scaled::take_scaled(pid, src, raw, frame);
    }
    let Some(read) = frame.get_mut(..src.read) else { return false };
    if !crate::linux::guest::Guest::read_into(pid, src.at, read) {
        return false;
    }
    if src.opaque {
        let (width, height, stride) = src.shape;
        super::super::opaque::make_opaque(frame, width, height, stride);
    }
    true
}
