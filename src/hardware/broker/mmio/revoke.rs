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

//! A holder's BAR mappings taken away before its device is reset:
//! unmapped from the holder's own address space, whichever process runs
//! here, with the TLB shot down on every CPU before this returns. A
//! thread that touches the BAR after that faults; none reaches a device
//! that is in reset.

use crate::hardware::broker::grant::{self, MmioGrant};
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::{
    lookup_asid_for_process, translate_in_asid, unmap_page_in_asid,
};
use crate::memory::paging::types::PagePermissions;

const PAGE: u64 = 4096;

/// The grants `pid` holds on `device_id`, out of the table.
pub(crate) fn take_for_device(pid: u32, device_id: u64) -> alloc::vec::Vec<MmioGrant> {
    grant::drain_for_device(pid, device_id)
}

/// Unmap each taken grant from `pid`'s address space. An address space
/// already gone maps nothing. The pages are a device's, never freed here.
pub(crate) fn revoke_taken(pid: u32, taken: &[MmioGrant]) {
    let Some(asid) = lookup_asid_for_process(pid) else {
        return;
    };
    let perms = PagePermissions::READ | PagePermissions::USER;
    for g in taken {
        for page in 0..g.length.div_ceil(PAGE) {
            let va = VirtAddr::new(g.user_va + page * PAGE);
            if translate_in_asid(asid, va).is_some() {
                let _ = unmap_page_in_asid(asid, va, perms);
            }
        }
    }
}
