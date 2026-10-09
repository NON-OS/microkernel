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

//! Turning a queue off, for a unit firmware handed over with QIE still set.
//! A queue cannot be re-pointed while enabled, and IQH only returns to zero
//! once it is off (VT-d 3.4, 11.4.9.1), so Linux disables before every
//! enable (intel/dmar.c, dmar_disable_qi) and so does this.

use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;
use crate::sys::serial::Line;

/// # Safety
/// The caller owns the unit's queue; nothing may submit while it is off.
pub(super) unsafe fn disable(unit: &RemapUnit) -> Result<(), VtdError> {
    // A refused descriptor firmware left stops the queue until its error
    // is cleared (write one to clear), and a stopped queue never drains.
    let stale =
        unit.read32(offsets::FSTS) & (offsets::FSTS_IQE | offsets::FSTS_ICE | offsets::FSTS_ITE);
    if stale != 0 {
        // SAFETY: eK@nonos.systems - clears status bits only.
        unsafe { unit.write32(offsets::FSTS, stale) };
    }
    // Cleared even with the queue off: a stale IQE would read as a refusal
    // of the kernel's first batch.
    if unit.read32(offsets::GSTS) & offsets::GSTS_QIES == 0 {
        return Ok(());
    }
    // Whatever firmware queued is let finish first. A queue still busy
    // after the budget is turned off anyway, as the SRTP-only write did
    // before queued invalidation, and the log says so.
    let drained =
        wait_ms(offsets::COMMAND_MS, || unit.read64(offsets::IQH) == unit.read64(offsets::IQT));
    if !drained {
        let mut line = Line::new();
        line.str(b"[VT-D] IOMMU unit base=").hex(unit.base_pa());
        line.str(b" firmware queue did not drain; turned off, errors cleared=");
        line.hex(stale as u64).end();
    }
    let command = offsets::gcmd_with(unit.read32(offsets::GSTS), 0) & !offsets::GCMD_QIE;
    // SAFETY: eK@nonos.systems - turning the queue off maps and unmaps
    // nothing; the caller holds it, so nothing of ours is pending.
    unsafe { unit.write32(offsets::GCMD, command) };
    if wait_ms(offsets::COMMAND_MS, || unit.read32(offsets::GSTS) & offsets::GSTS_QIES == 0) {
        Ok(())
    } else {
        Err(VtdError::Timeout)
    }
}
