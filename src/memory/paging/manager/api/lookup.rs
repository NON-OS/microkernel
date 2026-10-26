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

//! The lookups every context switch and system call makes, answered from
//! the directory (manager/directory) without the manager lock. Only when the
//! directory once ran out of room does a miss ask the manager itself.

use super::super::address_space::load_root;
use super::super::directory::{self, Entry};
use super::globals::PAGING_MANAGER;
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::smp::lock_responsive;

/// The lowest-asid address space `wanted` accepts, as (asid, pid, root).
pub(super) fn entry(wanted: impl Fn(&Entry) -> bool) -> Option<Entry> {
    directory::find(&wanted).or_else(|| {
        if directory::complete() {
            return None;
        }
        without_interrupts(|| {
            let mgr = lock_responsive(&PAGING_MANAGER);
            let all = mgr.address_spaces.iter();
            all.map(|(&asid, s)| (asid, s.process_id, s.cr3_value.as_u64())).find(|e| wanted(e))
        })
    })
}

pub fn lookup_asid_for_process(process_id: u32) -> Option<u32> {
    entry(|e| e.1 == process_id).map(|e| e.0)
}

pub fn get_process_cr3(process_id: u32) -> Option<u64> {
    entry(|e| e.1 == process_id).map(|e| e.2)
}

pub fn switch_address_space(asid: u32) -> PagingResult<()> {
    let (asid, _, root) = entry(|e| e.0 == asid).ok_or(PagingError::AddressSpaceNotFound)?;
    load_root(asid, root);
    Ok(())
}

pub fn switch_to_process_address_space(process_id: u32) -> PagingResult<()> {
    let found = entry(|e| e.1 == process_id).ok_or(PagingError::AddressSpaceNotFound)?;
    load_root(found.0, found.2);
    Ok(())
}
