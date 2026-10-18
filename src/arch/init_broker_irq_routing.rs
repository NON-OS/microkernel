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

/// Program the routes that carry device interrupts to a CPU, and report how
/// many were installed.
///
/// x86_64 reads the ACPI tables and fills in IO-APIC redirection entries, so the
/// count is the work done. aarch64's lines were routed when the GIC came up;
/// what is left is the MSI controller, whose tables need memory that did not
/// exist then. Without one PCI keeps INTx, which is not a failed stage.
pub(crate) fn init_broker_irq_routing() -> Result<usize, ()> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt::ioapic::init_from_acpi();

    #[cfg(target_arch = "aarch64")]
    return Ok(crate::arch::aarch64::gic::msi::bring_up_and_report());

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return Err(());
}
