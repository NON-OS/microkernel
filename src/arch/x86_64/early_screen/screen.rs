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

//! The firmware framebuffer through the loader's identity mapping, for the
//! panic and stop screens before init_arch_framebuffer maps it.

use super::frame::identity_frame_ok;
use crate::kernel_core::init::framebuffer::KernelFramebuffer;

pub(crate) fn early_screen() -> Option<KernelFramebuffer> {
    // Once clear_low_half has run the identity mapping is gone, and a write
    // through it would fault inside the panic path.
    if !crate::arch::x86_64::paging::boot_identity_live() {
        return None;
    }
    let fb = crate::boot::handoff::get_handoff()?.framebuffer()?;
    if !identity_frame_ok(fb.ptr, fb.size, fb.width, fb.height, fb.stride) {
        return None;
    }
    Some(KernelFramebuffer {
        width: fb.width,
        height: fb.height,
        stride: fb.stride,
        base_va: crate::memory::addr::VirtAddr::new(fb.ptr),
        offset: 0,
        bgr: matches!(fb.pixel_format, 1 | 3),
        physical_mm: fb.physical_mm(),
    })
}
