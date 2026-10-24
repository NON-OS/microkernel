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

//! Single 4 KiB frames for tables, queues and CDs, zeroed before the unit
//! can see them: a zero STE level 1 descriptor aborts, a zero page table
//! entry is not mapped, so zero is the safe initial state everywhere.

use crate::arch::aarch64::mmu::pointer::ram;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc::manager::{allocate_frame, deallocate_frame};
use crate::memory::iommu::IommuError;

pub(super) const WORDS: usize = 512;

pub(super) fn alloc() -> Result<u64, IommuError> {
    let phys = allocate_frame().ok_or(IommuError::PageTableExhausted)?.as_u64();
    match words(phys) {
        Ok(view) => {
            view.fill(0);
            Ok(phys)
        }
        Err(e) => {
            let _ = deallocate_frame(PhysAddr::new(phys));
            Err(e)
        }
    }
}

/// The kernel's view of a frame this module allocated.
pub(super) fn words(phys: u64) -> Result<&'static mut [u64; WORDS], IommuError> {
    let at = ram::<[u64; WORDS]>(phys).ok_or(IommuError::BackendFault)?;
    // SAFETY: `phys` is a 4 KiB frame `alloc` handed out and only this
    // driver writes it, under the state lock. The direct map that reaches it
    // lives as long as the kernel.
    Ok(unsafe { &mut *at })
}

/// Return a frame once nothing the unit can still walk points at it,
/// scrubbed first: page tables and CDs name physical memory, which the
/// next owner of the frame has no business reading.
pub(super) fn free(phys: u64) {
    if let Ok(view) = words(phys) {
        view.fill(0);
    }
    let _ = deallocate_frame(PhysAddr::new(phys));
}
