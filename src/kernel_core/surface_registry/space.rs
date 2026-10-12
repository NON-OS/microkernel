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
//! the thread group, and the tables it shares are registered to one of its
//! threads: the caller's own when the caller is of the group, else whichever
//! the group's id names.

use crate::memory::paging::manager::api::lookup_asid_for_process;

pub(crate) fn space_of(group: u32) -> Option<u32> {
    let mine = crate::process::current_pid()
        .filter(|&me| crate::hardware::broker::holder_of(me) == group)
        .and_then(lookup_asid_for_process);
    mine.or_else(|| lookup_asid_for_process(group))
}
