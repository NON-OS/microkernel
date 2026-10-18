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

use super::super::frame_len::frame_len;
use super::super::reshape::Shape;
use super::super::scene::Scene;
use super::super::scene_pixels::Pixels;
use super::surface::say;

const NO_POOL: &[u8] = b"[WAYLAND] commit of a buffer with no pool behind it, not shown\n";
const NO_FRAME: &[u8] =
    b"[WAYLAND] commit of a buffer empty, outside its pool or larger than a frame, not shown\n";

pub struct Source {
    /// Where the pixels start in the guest.
    pub at: u64,
    pub shape: Shape,
    /// How many bytes that is (`frame_len`).
    pub bytes: usize,
    /// XRGB: its alpha bytes are set as it is taken (opaque.rs).
    pub opaque: bool,
}

/// The pixels of `buffer`, or what to say instead.
pub fn source(scene: &Scene, buffer: u32) -> Result<Source, &'static [u8]> {
    let b = scene.buffers.iter().find(|b| b.id == buffer).ok_or(NO_POOL)?;
    let pool = scene.pools.iter().find(|p| p.id == b.pool).ok_or(NO_POOL)?;
    let at = pool.at.checked_add(b.offset).ok_or(NO_FRAME)?;
    let bytes = frame_len(b.offset, b.stride, b.height, pool.size).ok_or(NO_FRAME)?;
    Ok(Source { at, shape: (b.width, b.height, b.stride), bytes, opaque: b.opaque })
}

/// Read `src` out of the guest into `frame`, opaque when it is XRGB.
pub fn take(pid: u32, src: &Source, frame: &mut [u8]) -> bool {
    if !crate::linux::guest::Guest::read_into(pid, src.at, frame) {
        return false;
    }
    if src.opaque {
        let (width, height, stride) = src.shape;
        super::super::opaque::make_opaque(frame, width, height, stride);
    }
    true
}

/// `src` read out of the guest into pixels of its own, or None, said when
/// there is no room for them.
pub(super) fn taken(pid: u32, src: &Source) -> Option<Pixels> {
    let bytes = src.bytes;
    let Some(mut fresh) = Pixels::with_room(bytes) else {
        say(alloc::format!("[WAYLAND] no room for a {bytes} byte frame, not shown\n"));
        return None;
    };
    take(pid, src, fresh.frame(bytes)?).then_some(fresh)
}
