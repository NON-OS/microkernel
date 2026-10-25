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
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::sys::serial::{self, Line};

/// Clear a queue error and say what it was. On IQE the descriptor at the
/// head is the one refused (VT-d 6.5.2.10); it is overwritten with this
/// batch's wait, so the queue moves on and the wait still reports. The
/// refused batch is then an error to the caller.
pub(super) fn take_error(
    unit: &RemapUnit,
    ring_phys: u64,
    status_phys: u64,
    sequence: u32,
) -> Option<VtdError> {
    let status = unit.read32(offsets::FSTS);
    if status & offsets::FSTS_IQE != 0 {
        // A head past the ring is a failed unit, never an index.
        let Some(head) = offsets::queue_slot(unit.read64(offsets::IQH)) else {
            serial::println(b"[VT-D] IOMMU queue head past its ring; the unit has failed");
            return Some(VtdError::QueueHeadOutOfRange);
        };
        let head = head as usize;
        let replacement = wait(status_phys, sequence);
        let refused = Frame::at(ring_phys).ok().and_then(|ring| {
            let low = ring.read(head * 2).ok()?;
            ring.write(head * 2 + 1, replacement[1]).ok()?;
            ring.write(head * 2, replacement[0]).ok()?;
            Some(low)
        });
        // SAFETY: eK@nonos.systems - write-one-to-clear on a status bit; the
        // refused slot was replaced above, so the unit resumes on a wait.
        unsafe { unit.write32(offsets::FSTS, offsets::FSTS_IQE) };
        let mut line = Line::new();
        line.str(b"[VT-D] IOMMU queue refused descriptor low=").hex(refused.unwrap_or(0));
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
