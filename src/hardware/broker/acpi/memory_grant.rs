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

//! A SystemMemory window for the ACPI capsule: the rule over the whole pages
//! the window touches, then a mapping recorded in the broker's MMIO grant
//! table, so MkMmioUnmap and the capsule's exit take it back as they take
//! any other.

use super::error::RegionError;
use super::granting::GRANTING;
use super::kernel::records::{self, Space};
use super::kernel::Machine;
use super::memory_record::{record, MemoryGrant};
use super::report;
use super::rule::{self, Refusal};
use crate::hardware::broker::grant;
use crate::memory::addr::PhysAddr;

const PAGE: u64 = 0x1000;

/// The window [base, base + len) mapped into the caller, and where its first
/// byte landed. Refusals are logged; a window granted before is not again.
pub fn grant_memory(pid: u32, base: u64, len: u64) -> Result<MemoryGrant, RegionError> {
    let refuse = |r: Refusal| {
        report::refused(Space::Memory, base, len, r);
        RegionError::Refused(r)
    };
    if len == 0 {
        return Err(refuse(Refusal::Empty));
    }
    let end = base.checked_add(len).and_then(|e| e.checked_add(PAGE - 1));
    let (start, end) =
        (base & !(PAGE - 1), end.ok_or_else(|| refuse(Refusal::Wraps))? & !(PAGE - 1));
    let _granting = GRANTING.lock();
    rule::memory(start, end - start, &Machine { pid }).map_err(refuse)?;
    let fresh = !records::holds(pid, Space::Memory, start, end);
    if fresh && !records::insert(pid, Space::Memory, start, end) {
        return Err(RegionError::Full);
    }
    // From the caller's own MMIO window, given back if the map fails.
    let bytes = end - start;
    let va = grant::reserve_user_va(pid, bytes / PAGE).ok_or(RegionError::NoVaSpace)?;
    let undo = || {
        let _ = crate::memory::paging::unmap_user_mmio(va, bytes as usize);
        grant::release_user_va(pid, va.as_u64(), bytes);
    };
    let phys = PhysAddr::new(start);
    if crate::memory::paging::map_user_mmio(va, phys, bytes as usize, false).is_err() {
        undo();
        return Err(RegionError::MapFailed);
    }
    // A spent grant quota leaves nothing recorded, so nothing would unmap it.
    let Some(grant_id) = record(pid, start, end, va.as_u64()) else {
        undo();
        return Err(RegionError::Full);
    };
    if fresh {
        report::granted(Space::Memory, base, len);
    }
    Ok(MemoryGrant { user_va: va.as_u64() + (base - start), grant_id })
}
