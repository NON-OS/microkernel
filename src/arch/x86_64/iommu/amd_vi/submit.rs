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

//! Running commands on a unit and waiting for them, as Linux does in
//! amd/iommu.c (iommu_queue_command_sync, then iommu_completion_wait): the
//! commands, a completion wait behind them, the tail, then the store polled.

use core::sync::atomic::{compiler_fence, Ordering};

use super::command::{command_opcode, completion_wait, Command};
use super::error::AmdViError;
use super::regs;
use super::ring::RINGS;
use super::units::units;
use crate::arch::x86_64::iommu::regs::offsets::COMMAND_MS;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;

/// Returns once unit `index` stored this batch's sequence, which it does
/// only after every command before the wait completed.
pub(super) fn submit(index: usize, batch: &[Command]) -> Result<(), AmdViError> {
    let unit = units().get(index).ok_or(AmdViError::NotPresent)?;
    let mut ring = RINGS.get(index).ok_or(AmdViError::NotPresent)?.lock();
    if ring.phys == 0 {
        return Err(AmdViError::NotPresent);
    }
    let head = regs::ring_index(unit.read64(regs::CMD_HEAD).ok_or(AmdViError::Timeout)?);
    let free = (head + regs::RING_ENTRIES - ring.tail - 1) % regs::RING_ENTRIES;
    if (free as usize) < batch.len() + 1 {
        return Err(AmdViError::Timeout);
    }
    ring.sequence = ring.sequence.wrapping_add(1).max(1);
    let sequence = ring.sequence;
    let wait = completion_wait(ring.store_phys, sequence);
    let slots = entries_mut(ring.phys).map_err(|_| AmdViError::TableUnreachable)?;
    for command in batch.iter().chain(core::iter::once(&wait)) {
        let at = ring.tail as usize * 2;
        slots[at] = ((command[1] as u64) << 32) | command[0] as u64;
        slots[at + 1] = ((command[3] as u64) << 32) | command[2] as u64;
        ring.tail = regs::ring_next(ring.tail);
    }
    compiler_fence(Ordering::SeqCst);
    // SAFETY: eK@nonos.systems - every slot below the new tail holds a command
    // written above; the commands only invalidate caches.
    unsafe { unit.write64(regs::CMD_TAIL, regs::ring_offset(ring.tail)) };
    let store = entries_mut(ring.store_phys).map_err(|_| AmdViError::TableUnreachable)?;
    let store = store.as_ptr();
    // SAFETY: eK@nonos.systems - the first quadword of the store page this
    // module owns; the unit writes it, so it is read volatile.
    let done = wait_ms(COMMAND_MS, || unsafe { core::ptr::read_volatile(store) } == sequence);
    if done {
        Ok(())
    } else {
        if let Some(first) = batch.first() {
            crate::log::warn!("[amd-vi] command opcode {} did not complete in time", command_opcode(*first));
        }
        Err(AmdViError::Timeout)
    }
}
