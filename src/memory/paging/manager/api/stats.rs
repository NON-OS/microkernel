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

use super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::layout;
use crate::memory::paging::types::PagingStats;
use crate::smp::lock_responsive;

pub fn get_paging_stats() -> PagingStats {
    let counts = without_interrupts(|| {
        let manager = lock_responsive(&PAGING_MANAGER);
        (manager.mappings_count(), manager.address_spaces_count())
    });
    PAGING_STATS.snapshot(counts.0, counts.1)
}

pub fn get_memory_usage() -> (usize, usize) {
    let stats = get_paging_stats();
    (stats.user_pages * layout::PAGE_SIZE, stats.kernel_pages * layout::PAGE_SIZE)
}
