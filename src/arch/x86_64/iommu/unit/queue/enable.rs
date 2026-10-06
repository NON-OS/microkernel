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

//! Putting a unit's invalidations on its queue (VT-d 3.4, 6.5.2), the way
//! Linux does in intel/dmar.c, __dmar_enable_qi: tail to zero, the queue
//! address, then QIE, then wait for QIES.

use super::disable::disable;
use super::state::slot_of;
use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;

/// Start the unit's queue. `Ok(false)` when the unit has none (ECAP.QI
/// clear), which leaves it on register-based invalidation.
///
/// # Safety
/// Called from bring-up only, before the unit translates with these tables.
pub unsafe fn enable(unit: &RemapUnit) -> Result<bool, VtdError> {
    if !cap::queued_invalidation(unit.read64(offsets::ECAP)) {
        return Ok(false);
    }
    let slot = slot_of(unit).ok_or(VtdError::NotPresent)?;
    let mut queue = slot.lock();
    queue.live = false;
    // SAFETY: eK@nonos.systems - the lock is held, so nothing submits.
    unsafe { disable(unit)? };
    if queue.ring_phys == 0 {
        // Kept across a second enable: a page the unit was pointed at is
        // never handed back, since a late fetch could read its next owner.
        queue.ring_phys = allocate_table()?;
        queue.status_phys = allocate_table()?;
    }
    queue.tail = 0;
    // SAFETY: eK@nonos.systems - the ring is a page this module owns for the
    // life of the kernel, and with the tail at zero the unit fetches nothing.
    unsafe {
        unit.write64(offsets::IQT, 0);
        unit.write64(offsets::IQA, offsets::iqa_value(queue.ring_phys));
        let command = offsets::gcmd_with(unit.read32(offsets::GSTS), offsets::GCMD_QIE);
        unit.write32(offsets::GCMD, command);
    }
    if !wait_ms(offsets::COMMAND_MS, || unit.read32(offsets::GSTS) & offsets::GSTS_QIES != 0) {
        return Err(VtdError::Timeout);
    }
    queue.live = true;
    Ok(true)
}
