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

//! The errors a unit raises while it works through its queue, in Fault
//! Status (VT-d 3.4, 6.5.2.10). Every one stops the queue until software
//! clears it, so a wait that ignored them would spin out its whole budget.

use super::descriptor::wait;
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::sys::serial::Line;

/// Clear a queue error and say what it was. On IQE the descriptor at the
/// head is the one refused; it is overwritten with this batch's wait, as
/// Linux does in intel/dmar.c, qi_check_fault, so the queue moves on and the
/// wait still reports. The refused batch is then an error to the caller.
pub(super) fn take_error(
    unit: &RemapUnit,
    ring_phys: u64,
    status_phys: u64,
    sequence: u32,
) -> Option<VtdError> {
    let status = unit.read32(offsets::FSTS);
    if status & offsets::FSTS_IQE != 0 {
        let head = offsets::queue_index(unit.read64(offsets::IQH)) as usize;
        let refused = match entries_mut(ring_phys) {
            Ok(ring) => {
                let low = ring[head * 2];
                let replacement = wait(status_phys, sequence);
                ring[head * 2 + 1] = replacement[1];
                ring[head * 2] = replacement[0];
                low
            }
            Err(_) => 0,
        };
        // SAFETY: eK@nonos.systems - write-one-to-clear on a status bit; the
        // refused slot was replaced above, so the unit resumes on a wait.
        unsafe { unit.write32(offsets::FSTS, offsets::FSTS_IQE) };
        let mut line = Line::new();
        line.str(b"[VT-D] IOMMU queue refused descriptor low=").hex(refused);
        line.str(b" at slot ").dec(head as u64).end();
        return Some(VtdError::InvalidationRejected);
    }
    // Device TLB timeouts and completion errors belong to ATS invalidations,
    // which this kernel never queues; one appearing is reported and cleared
    // so the queue keeps running.
    let other = status & (offsets::FSTS_ICE | offsets::FSTS_ITE);
    if other != 0 {
        // SAFETY: eK@nonos.systems - write-one-to-clear on status bits.
        unsafe { unit.write32(offsets::FSTS, other) };
        let mut line = Line::new();
        line.str(b"[VT-D] IOMMU queue completion error fsts=").hex(other as u64).end();
    }
    None
}
