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

//! Bring-up that cannot go on says why before the CPU parks.
//!
//! Each refusal is one `[BOOT]` line naming the step and the reason, written
//! through the fatal writer so it does not wait on a lock, then the CPU halts
//! with interrupts still masked as the boot path leaves them.

use crate::arch::aarch64::cpu;
use crate::arch::aarch64::cpu::id::mpidr_affinity;
use crate::arch::aarch64::security::pac::PacError;
use crate::sys::serial::Line;

/// Park this CPU after naming the step that failed and why.
pub(crate) fn refuse(step: &[u8], reason: &[u8]) -> ! {
    Line::new()
        .str(b"[BOOT] refused: ")
        .str(step)
        .str(b": ")
        .str(reason)
        .str(b", mpidr=")
        .hex(mpidr_affinity())
        .end_fatal();
    cpu::halt()
}

/// What the security bring-up error means, as a reason for [`refuse`].
pub(crate) fn security_reason(error: PacError) -> &'static [u8] {
    match error {
        PacError::EntropyUnavailable => b"no entropy source to key pointer authentication from",
    }
}
