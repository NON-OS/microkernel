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

//! The address space a grant's holder runs in, and its grants' pages taken
//! out of it. A holder can be a thread: it runs in its process's
//! tables and owns no address-space entry, so it is found by its tables.
//! Found either way, the pages come out from any context, with the TLB
//! shot down on every CPU running those tables before this returns.

use core::sync::atomic::Ordering;

use super::holder_choice::{choose, Space};
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::{
    lookup_asid_for_cr3, lookup_asid_for_process, translate_in_asid, unmap_page_in_asid,
};
use crate::memory::paging::types::PagePermissions;

const PAGE: u64 = 4096;

/// The asid `pid` runs in: its own, or, for a thread, its process's. None
/// is holder_choice's active-table fallback, which callers take themselves.
pub(crate) fn holder_asid(pid: u32) -> Option<u32> {
    let cr3 =
        || crate::process::PROCESS_TABLE.find_by_pid(pid).map(|p| p.cr3.load(Ordering::Acquire));
    match choose(lookup_asid_for_process(pid), cr3, lookup_asid_for_cr3) {
        Space::Asid(asid) => Some(asid),
        Space::ActiveTable => None,
    }
}

/// Unmap `length` bytes at `user_va` from `pid`'s tables. False when they
/// are gone: then nothing maps the range any more.
pub(crate) fn unmap_from_holder(pid: u32, user_va: u64, length: u64) -> bool {
    let Some(asid) = holder_asid(pid) else {
        return false;
    };
    for page in 0..length.div_ceil(PAGE) {
        let va = VirtAddr::new(user_va + page * PAGE);
        if translate_in_asid(asid, va).is_some() {
            let _ = unmap_page_in_asid(asid, va, PagePermissions::USER);
        }
    }
    true
}
