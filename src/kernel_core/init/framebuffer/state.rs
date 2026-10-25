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

use crate::memory::addr::VirtAddr;
use spin::Once;

#[derive(Clone, Copy)]
pub(crate) struct KernelFramebuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub base_va: VirtAddr,
    pub offset: usize,
    pub bgr: bool,
    /// The panel's (width, height) in millimetres from its EDID, if known.
    pub physical_mm: Option<(u32, u32)>,
}

impl KernelFramebuffer {
    pub(crate) fn frame_len(self) -> Option<usize> {
        (self.stride as usize).checked_mul(self.height as usize)
    }

    /// Framebuffer pixels per console pixel, by the rule in hidpi_scale.
    pub(crate) fn hidpi_scale(self) -> u32 {
        super::hidpi::hidpi_scale(self.width, self.height, self.physical_mm)
    }
}

pub(super) static FRAMEBUFFER: Once<KernelFramebuffer> = Once::new();
/// The frame's physical range, `(start, pitch * height)`: what a device
/// window must contain to hold the boot framebuffer (broker::boot_fb_held).
pub(super) static FRAME_PHYS: Once<(u64, u64)> = Once::new();
/// Why the loader handed over none, as FbRefusal::why_word says it.
pub(super) static REFUSAL: Once<u32> = Once::new();

pub(crate) fn framebuffer_phys() -> Option<(u64, u64)> {
    FRAME_PHYS.get().copied()
}

pub(crate) fn framebuffer_state() -> Option<&'static KernelFramebuffer> {
    FRAMEBUFFER.get()
}

/// Why there is no boot framebuffer, for nonos_display_refusal: the loader's
/// reason, or WHY_NOT_MAPPED when it handed one over the kernel refused.
pub(crate) fn boot_refusal() -> u32 {
    let not_mapped = crate::boot::handoff::types::refusal::WHY_NOT_MAPPED as u32;
    REFUSAL.get().copied().unwrap_or(not_mapped)
}

/// The boot framebuffer's length in bytes (stride x height), its geometry
/// alone: what the surface cap is sized by (surface_handlers/cap.rs).
pub(crate) fn boot_frame_len() -> Option<usize> {
    framebuffer_state().and_then(|f| f.frame_len())
}
