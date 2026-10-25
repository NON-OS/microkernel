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

//! Installing the copy as the boot handoff every shared reader asks for
//! (`boot::handoff::get_handoff`), through the same checks x86_64's goes
//! through: entropy, memory map, framebuffer and entry point.

use super::take::COPY;
use crate::boot::handoff::{init_handoff, BootHandoffV1, HandoffError};
use crate::sys::serial::Line;

/// The installed handoff, `None` with no loader. Run where the kernel will
/// keep running, after the move to the high half: the shared code keeps a
/// reference to the copy, which must not move again.
pub(in crate::arch::aarch64::boot) fn install(
) -> Result<Option<&'static BootHandoffV1>, HandoffError> {
    let Some(copy) = COPY.get() else {
        return Ok(None);
    };
    // SAFETY: `copy` is a kernel static, aligned and live for the kernel's
    // life, at the address it now runs at.
    let handoff = unsafe { init_handoff(copy as *const BootHandoffV1 as u64)? };
    Line::new()
        .str(b"[BOOT] NONOS loader handoff taken: kernel verified=")
        .dec(u64::from(handoff.kernel_verified()))
        .str(b" secure boot=")
        .dec(u64::from(handoff.secure_boot_enabled()))
        .end();
    Ok(Some(handoff))
}
