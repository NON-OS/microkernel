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

//! Handing a batch of descriptors to a unit and waiting until it is done:
//! the batch, an invalidation wait descriptor behind it (VT-d 6.5.2.8), the
//! new tail, then the status dword polled, since the queue runs in order.

use core::sync::atomic::{compiler_fence, Ordering};

use super::descriptor::{wait, Descriptor};
use super::finish::finish;
use super::state::slot_of;
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// Returns once the unit wrote this batch's status, which it does only after
/// every descriptor in the batch completed. The lock is held throughout, so
/// batches on one unit never interleave.
pub fn submit(unit: &RemapUnit, batch: &[Descriptor]) -> Result<(), VtdError> {
    let slot = slot_of(unit).ok_or(VtdError::NotPresent)?;
    let mut queue = slot.lock();
    if !queue.live {
        return Err(VtdError::NotPresent);
    }
    // A queue the unit stopped draining has no room; that is a unit that
    // stopped answering, and waiting longer would not change it.
    let Some(head) = offsets::queue_slot(unit.read64(offsets::IQH)) else {
        return Err(VtdError::QueueHeadOutOfRange);
    };
    if (offsets::queue_free(head, queue.tail) as usize) < batch.len() + 1 {
        return Err(VtdError::Timeout);
    }
    queue.sequence = queue.sequence.wrapping_add(1).max(1);
    let sequence = queue.sequence;
    let fence = wait(queue.status_phys, sequence);
    let ring = Frame::at(queue.ring_phys)?;
    // The tail moves only once the whole batch is written, so a refused write
    // never leaves half a batch behind the next one.
    let mut tail = queue.tail;
    for descriptor in batch.iter().chain(core::iter::once(&fence)) {
        let at = tail as usize * 2;
        ring.write(at, descriptor[0])?;
        ring.write(at + 1, descriptor[1])?;
        tail = offsets::queue_next(tail);
    }
    queue.tail = tail;
    // Descriptors in memory before the tail that lets the unit fetch them.
    compiler_fence(Ordering::SeqCst);
    // SAFETY: eK@nonos.systems - every slot below the new tail holds a
    // descriptor written above; invalidations only narrow what devices reach.
    unsafe { unit.write64(offsets::IQT, offsets::queue_offset(queue.tail)) };

    finish(unit, queue.ring_phys, queue.status_phys, sequence)
}
