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

//! Each unit's event log: one page of 16-byte events the unit appends at
//! its tail and the kernel consumes from its head (spec 48882, 2.5).

use core::sync::atomic::{AtomicU64, Ordering};

use super::error::AmdViError;
use super::mmio::Unit;
use super::regs;
use crate::arch::x86_64::acpi::parser::other::ivrs_walk::MAX_AMD_IOMMUS;
use crate::arch::x86_64::iommu::tables::frame::allocate_table;

const NONE: AtomicU64 = AtomicU64::new(0);

/// The event log page of each unit, zero before bring-up.
pub(super) static LOGS: [AtomicU64; MAX_AMD_IOMMUS] = [NONE; MAX_AMD_IOMMUS];

/// Point the unit at a fresh event log with head and tail at zero, as Linux
/// does in amd/init.c (iommu_enable_event_buffer) before EVENT_LOG_EN.
///
/// # Safety
/// The unit must be stopped.
pub(super) unsafe fn attach_log(index: usize, unit: &Unit) -> Result<(), AmdViError> {
    let slot = LOGS.get(index).ok_or(AmdViError::NotPresent)?;
    let mut phys = slot.load(Ordering::Acquire);
    if phys == 0 {
        phys = allocate_table().map_err(|_| AmdViError::NoFrames)?;
        slot.store(phys, Ordering::Release);
    }
    // SAFETY: eK@nonos.systems - the unit is stopped, so nothing is appended
    // while the base and pointers change; the page lives as long as the kernel.
    unsafe {
        unit.write64(regs::EVENT_LOG_BASE, regs::ring_base(phys));
        unit.write64(regs::EVENT_HEAD, 0);
        unit.write64(regs::EVENT_TAIL, 0);
    }
    Ok(())
}
