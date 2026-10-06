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

use crate::memory::paging::manager::api::lookup_asid_for_process;
use crate::process::{current_pid, current_process};

/*
 * VA for an `addr == 0` mmap: a range from the per-process allocator that
 * holds no present page, so the mapping never overwrites the image, the
 * stack or any other live mapping of the caller.
 */
pub(crate) fn reserve_va(pages: u64) -> Option<u64> {
    let asid = lookup_asid_for_process(current_pid()?)?;
    current_process()?.reserve_unmapped(pages, asid)
}
