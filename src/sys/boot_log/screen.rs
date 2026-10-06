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

//! Which framebuffer the panic and stop screens draw on: the kernel's own
//! mapping once init_arch_framebuffer made it, and before that the
//! loader's identity mapping of the firmware framebuffer. Without the
//! second, a machine that stopped in early boot left the loader's splash
//! on the panel and said where it stopped only on a serial port most
//! laptops do not have.

use crate::kernel_core::init::framebuffer::{framebuffer_state, KernelFramebuffer};

pub(super) fn screen() -> Option<KernelFramebuffer> {
    if let Some(fb) = framebuffer_state() {
        return Some(*fb);
    }
    early_screen()
}

#[cfg(target_arch = "x86_64")]
fn early_screen() -> Option<KernelFramebuffer> {
    use super::early_frame::identity_frame_ok;
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

#[cfg(not(target_arch = "x86_64"))]
fn early_screen() -> Option<KernelFramebuffer> {
    None
}
