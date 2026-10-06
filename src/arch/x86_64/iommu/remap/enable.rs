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

//! Turning interrupt remapping on for one unit, in the order of Linux
//! intel/irq_remapping.c (iommu_set_irq_remapping, then
//! iommu_enable_irq_remapping): table address, SIRTP, a global interrupt
//! entry cache invalidation, then IRE. CFI goes on before IRE, so there is
//! no moment in which a compatibility format interrupt is blocked.

use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;
use crate::arch::x86_64::iommu::unit::queue;

/// # Safety
/// `irta` must name a table this kernel owns for its whole life, and the
/// unit's invalidation queue must be live: the entry cache can only be
/// invalidated through it.
pub(super) unsafe fn enable_on(unit: &RemapUnit, irta: u64) -> Result<(), VtdError> {
    // SAFETY: eK@nonos.systems - the caller's promise about the table; until
    // SIRTP completes the unit keeps whatever pointer it had, and until IRE
    // is set it reads none of it.
    unsafe {
        unit.write64(offsets::IRTA, irta);
        command(unit, offsets::GCMD_SIRTP, offsets::GSTS_IRTPS)?;
    }
    queue::flush_interrupt_entries(unit)?;
    // SAFETY: eK@nonos.systems - CFI lets compatibility format interrupts
    // through exactly as they arrive today, and IRE then only adds the
    // remapped entries this table holds, none of them present yet.
    unsafe {
        command(unit, offsets::GCMD_CFI, offsets::GSTS_CFIS)?;
        command(unit, offsets::GCMD_IRE, offsets::GSTS_IRES)
    }
}

/// # Safety
/// As `enable_on`: the bit changes how the unit treats interrupts.
unsafe fn command(unit: &RemapUnit, bit: u32, done: u32) -> Result<(), VtdError> {
    let value = offsets::gcmd_with(unit.read32(offsets::GSTS), bit);
    // SAFETY: eK@nonos.systems - every persistent control is carried from
    // GSTS, so this write changes `bit` and nothing else.
    unsafe { unit.write32(offsets::GCMD, value) };
    if wait_ms(offsets::COMMAND_MS, || unit.read32(offsets::GSTS) & done != 0) {
        Ok(())
    } else {
        Err(VtdError::Timeout)
    }
}
