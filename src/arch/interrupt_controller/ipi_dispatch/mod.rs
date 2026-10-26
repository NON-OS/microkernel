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

//! Getting IPI handlers attached to the interrupt controller.
//!
//! Attaching is the arch-specific half: x86_64 installs IDT gates while the
//! table is built, aarch64 registers SGIs with the GIC once it is up. Both end
//! up calling the same handlers. riscv64 has no IPIs here (no interrupt
//! controller backend sends any), so it has no handlers at all.

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod handlers;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64::install_gates;

/// From the boot CPU's SMP init, once its controller is up. The GIC's SGIs
/// are registered here; x86_64's gates went into the one IDT as it was built,
/// earlier, the only point every CPU loads a table holding them; riscv64 has
/// none to register.
pub(crate) fn register_ipi_handlers() {
    #[cfg(target_arch = "aarch64")]
    aarch64::register_ipi_handlers();
}
