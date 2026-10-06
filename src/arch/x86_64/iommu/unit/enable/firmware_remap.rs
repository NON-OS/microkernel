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

//! Interrupt remapping firmware left on. Before the queue change, setting the
//! root pointer wrote GCMD with SRTP alone, which also cleared IRE; now that
//! GCMD keeps every persistent control, IRE must be turned off by name, as
//! Linux does when it finds remapping on (intel/irq_remapping.c,
//! intel_setup_irq_remapping, then iommu_disable_irq_remapping). Left on
//! with firmware's table and CFI clear, every compatibility format MSI and
//! I/O APIC interrupt the kernel programs is blocked (fault reason 0x25).

use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;
use crate::sys::serial::Line;

/// Turn IRE off if firmware left it on, and say what was found.
///
/// # Safety
/// Bring-up only, before the queue is touched: IRE must be off before QIE
/// is, since the unit invalidates its interrupt entry cache through the queue.
pub(super) unsafe fn release_remapping(unit: &RemapUnit) {
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU unit base=").hex(unit.base_pa());
    let status = unit.read32(offsets::GSTS);
    if status & offsets::GSTS_IRES == 0 {
        line.str(b" firmware interrupt remapping=off").end();
        return;
    }
    let command = offsets::gcmd_with(status, 0) & !offsets::GCMD_IRE;
    // SAFETY: eK@nonos.systems - with IRE off the unit passes interrupts as
    // the kernel programs them, which is how the photographed image ran.
    unsafe { unit.write32(offsets::GCMD, command) };
    let off = wait_ms(offsets::COMMAND_MS, || unit.read32(offsets::GSTS) & offsets::GSTS_IRES == 0);
    line.str(if off {
        b" firmware interrupt remapping=on; turned off"
    } else {
        b" firmware interrupt remapping=on; would not turn off"
    });
    line.end();
}
