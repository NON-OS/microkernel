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

use crate::kernel_core::init::framebuffer::KernelFramebuffer;

/// The firmware framebuffer as the loader left it mapped, for a stop screen
/// drawn before the kernel maps its own.
///
/// x86_64: the loader's identity mapping, while it lasts. aarch64 and
/// riscv64 have none: no loader there maps a framebuffer for the kernel, and
/// a Pi's is asked of the firmware only once paging is up.
pub(crate) fn early_screen() -> Option<KernelFramebuffer> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::early_screen::early_screen();

    #[cfg(not(target_arch = "x86_64"))]
    return None;
}
