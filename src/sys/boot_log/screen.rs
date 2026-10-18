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
    crate::arch::early_screen()
}
