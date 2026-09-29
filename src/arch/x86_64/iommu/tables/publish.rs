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

//! Making a table write visible to a unit that does not snoop CPU caches.

use crate::arch::x86_64::iommu::regs::cap::page_walk_coherent;
use crate::arch::x86_64::iommu::unit::report::probed;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

const LINE: usize = 64;
const PAGE: usize = 4096;

/// Flush the table page at `table_phys` when the unit's walks are not
/// coherent. Without it a unit reading memory directly sees the entry's
/// previous value: a mapping that never appears, or an unmap that never
/// happens while the frame behind it is reused.
pub fn publish(table_phys: u64) {
    let Some(info) = probed() else {
        return;
    };
    if page_walk_coherent(info.ecap) {
        return;
    }
    let Some(virt) = phys_to_virt(PhysAddr::new(table_phys)) else {
        return;
    };
    for offset in (0..PAGE).step_by(LINE) {
        let line = virt.as_u64() as usize + offset;
        // SAFETY: eK@nonos.systems - `line` lies inside the directmap view of a
        // table frame this module owns; clflush writes back and drops the line
        // and changes no memory contents.
        unsafe {
            core::arch::asm!("clflush [{}]", in(reg) line, options(nostack, preserves_flags));
        }
    }
    // SAFETY: eK@nonos.systems - a fence has no memory operands; it orders the
    // flushes above before whatever invalidation the caller issues next.
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
}
