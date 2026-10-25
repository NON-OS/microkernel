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

//! The firmware framebuffer as the loader left it, for the panic and stop
//! screens before the kernel maps its own. Plain numbers in, so
//! kernel_proofs runs this file on the host.

/// nonos-bootloader/src/paging/fb_window.rs IDENTITY_FB_LIMIT: the loader
/// identity maps a framebuffer that ends below this, and nothing past it.
pub(crate) const IDENTITY_FB_LIMIT: u64 = 1 << 47;

/// Whether every pixel `y * stride + x * 4` with x below `width` and y
/// below `height` lies inside [ptr, ptr + size) and inside the identity
/// window. `stride` is the row pitch in bytes, as the handoff carries it.
pub(crate) fn identity_frame_ok(ptr: u64, size: u64, width: u32, height: u32, stride: u32) -> bool {
    if ptr == 0 || width == 0 || height == 0 || (stride as u64) < width as u64 * 4 {
        return false;
    }
    let frame = stride as u64 * height as u64;
    if frame > size {
        return false;
    }
    matches!(ptr.checked_add(frame), Some(end) if end <= IDENTITY_FB_LIMIT)
}
