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
use crate::memory::paging::PagePermissions;

use super::super::consts::PAGE_SIZE;
use super::map_kernel::Kernel;
use super::map_loop::map_loop;

/*
 * Back `pages` pages from `base` with fresh frames, zeroed before each is
 * mapped, interrupts let in every WINDOW_PAGES pages (map_loop.rs). On
 * failure the pages this call mapped are unmapped and freed, and false is
 * returned. The range is busy throughout (`pin::busy`): a sibling thread
 * could otherwise register pages the rollback then frees.
 */
pub(in super::super) fn map_fresh(base: u64, pages: u64, perms: PagePermissions) -> bool {
    let len = pages * PAGE_SIZE as u64;
    let _busy = crate::kernel_core::surface_registry::pin::busy::begin(base, len);
    map_loop(&mut Kernel { base, perms }, pages as usize)
}
