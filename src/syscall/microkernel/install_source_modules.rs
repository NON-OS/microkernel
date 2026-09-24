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

//! The module table the loader hands over, read through the directmap.
//!
//! The loader records the table by its physical address, as it does every
//! other handoff pointer. Dereferencing that address as a kernel pointer
//! faulted in ring 0 on the first `MkInstallSource` call, so the table is
//! translated here, with its last byte checked as well as its first.

use crate::boot::handoff::get_handoff;
use crate::boot::handoff::types::Module;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

/// The first recorded module of `kind` with a non-empty image, by value.
pub fn find(kind: u64) -> Option<Module> {
    let handoff = get_handoff()?;
    let count = handoff.modules.count as usize;
    let bytes = (count * core::mem::size_of::<Module>()) as u64;
    if bytes == 0 {
        return None;
    }
    let first = handoff.modules.ptr;
    let last = first.checked_add(bytes - 1)?;
    phys_to_virt(PhysAddr::new(last))?;
    let virt = phys_to_virt(PhysAddr::new(first))?;
    // SAFETY: eK@nonos.systems - the loader wrote this array into loader
    // memory before the jump, the kernel never frees loader memory, and both
    // ends of the array were just checked to lie under the directmap.
    let modules = unsafe { core::slice::from_raw_parts(virt.as_u64() as *const Module, count) };
    modules.iter().copied().find(|m| m.kind as u64 == kind && m.size > 0)
}
