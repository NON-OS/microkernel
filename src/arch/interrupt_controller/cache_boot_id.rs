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

//! Remember what the controller calls the boot CPU.
//!
//! Read once during init and kept, because later callers want it from
//! contexts where reading the controller is either expensive or not allowed:
//! interrupt routing needs a destination while interrupts are masked, and
//! teardown wants it after the local controller has been quiesced.
//!
//! aarch64 needs no cache. `MPIDR_EL1` is a plain system register read with
//! no memory-mapped access behind it, so asking is already as cheap as
//! remembering.

/// Latch the boot CPU's controller id for later use.
pub(crate) fn cache_boot_cpu_id() {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::interrupt::apic::cache_bsp_apic_id();
    }
}

/// The boot CPU's controller id, latched before the SMP layer records it.
/// On x86_64 it is checked against CPUID, which `cpu_id()` resolves by, and
/// a disagreement fails the boot rather than strand the boot CPU later.
pub(crate) fn latch_boot_cpu_id() -> Result<u32, &'static str> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt::apic::latch_boot_id();
    #[cfg(not(target_arch = "x86_64"))]
    return Ok(super::local_id());
}
