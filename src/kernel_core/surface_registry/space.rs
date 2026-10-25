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

//! The address space a holder's mappings live in (L36). Surfaces are held by
//! the thread group, and the tables it shares are found from a live member's
//! own `cr3` (L46): the owner of those tables changes when the first owner
//! exits, so a lookup by the caller's pid or the group's id could miss while
//! the tables still mapped the frames.

use core::sync::atomic::Ordering;

use crate::hardware::broker::holder_of;
use crate::memory::paging::manager::api::lookup_asid_for_tables;
use crate::process::address_space::lifecycle::holder::released;
use crate::process::core::PROCESS_TABLE;

/// The tables `group` runs on: the caller's when it is of the group, else
/// those of any of its threads still in the process table.
pub(crate) fn tables_of(group: u32) -> Option<u64> {
    let cr3 = |pid: u32| PROCESS_TABLE.find_by_pid(pid).map(|p| p.cr3.load(Ordering::Acquire));
    let mine = crate::process::current_pid().filter(|&me| holder_of(me) == group);
    mine.and_then(cr3).filter(|&t| t != 0).or_else(|| {
        let all = PROCESS_TABLE.get_all_processes();
        all.iter()
            // A released thread's `cr3` may name tables already freed (L49).
            .filter(|p| holder_of(p.pid) == group && !released(p))
            .map(|p| p.cr3.load(Ordering::Acquire))
            .find(|&t| t != 0)
    })
}

pub(crate) fn space_of(group: u32) -> Option<u32> {
    tables_of(group).and_then(lookup_asid_for_tables)
}
