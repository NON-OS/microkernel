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

pub(crate) fn framebuffer_state() -> Option<&'static KernelFramebuffer> {
    FRAMEBUFFER.get()
}
