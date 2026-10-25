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

//! Whether the two names the boot CPU goes by agree. Pure, so the host
//! proofs hold it.
//!
//! `cpu_id()` finds a CPU's descriptor by the id CPUID reports (leaf 0xB
//! when the part has it), and IPIs and device interrupts are addressed by the
//! id the local APIC answers to. The boot CPU's descriptor is filled from one
//! of them, so they have to be the same number or the boot CPU cannot find
//! itself, or cannot be reached.

/// The id to record for the boot CPU, or both ids when they differ.
///
/// `controller` is the local APIC's own id register, `None` before the LAPIC
/// is up, when CPUID is all there is. In xAPIC mode that register holds eight
/// bits, so a CPUID id past 255 cannot be the same CPU's register at all.
pub const fn reconcile(
    cpuid: u32,
    controller: Option<u32>,
    x2apic: bool,
) -> Result<u32, (u32, u32)> {
    let Some(controller) = controller else {
        return Ok(cpuid);
    };
    if !x2apic && cpuid > 0xFF {
        return Err((cpuid, controller));
    }
    if cpuid == controller {
        Ok(cpuid)
    } else {
        Err((cpuid, controller))
    }
}
