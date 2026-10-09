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

//! Each unit's command buffer: one page of commands, and a page whose first
//! quadword a completion wait stores into.

use spin::Mutex;

use super::error::AmdViError;
use super::mmio::Unit;
use super::regs;
use crate::arch::x86_64::acpi::parser::other::ivrs_walk::MAX_AMD_IOMMUS;
use crate::arch::x86_64::iommu::tables::frame::allocate_table;

pub(super) struct Ring {
    pub phys: u64,
    pub store_phys: u64,
    pub tail: u16,
    /// The value the next completion wait stores, never zero, so a fresh
    /// store page cannot read as an answer.
    pub sequence: u64,
}

const IDLE: Mutex<Ring> = Mutex::new(Ring { phys: 0, store_phys: 0, tail: 0, sequence: 0 });

pub(super) static RINGS: [Mutex<Ring>; MAX_AMD_IOMMUS] = [IDLE; MAX_AMD_IOMMUS];

/// Point the unit at a fresh command buffer with head and tail at zero, as
/// Linux does in amd/init.c (iommu_enable_command_buffer) before CMD_BUF_EN.
///
/// # Safety
/// The unit must be stopped (IOMMU_EN and CMD_BUF_EN clear).
pub(super) unsafe fn attach_ring(index: usize, unit: &Unit) -> Result<(), AmdViError> {
    let slot = RINGS.get(index).ok_or(AmdViError::NotPresent)?;
    let mut ring = slot.lock();
    if ring.phys == 0 {
        ring.phys = allocate_table().map_err(|_| AmdViError::NoFrames)?;
        ring.store_phys = allocate_table().map_err(|_| AmdViError::NoFrames)?;
    }
    ring.tail = 0;
    // SAFETY: eK@nonos.systems - the unit is stopped, so it fetches nothing
    // while the base and pointers change; the page lives as long as the kernel.
    unsafe {
        unit.write64(regs::CMD_BUF_BASE, regs::ring_base(ring.phys));
        unit.write64(regs::CMD_HEAD, 0);
        unit.write64(regs::CMD_TAIL, 0);
    }
    Ok(())
}
