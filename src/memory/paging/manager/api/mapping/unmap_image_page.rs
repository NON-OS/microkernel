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

use super::super::globals::PAGING_MANAGER;
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::PagingResult;
use crate::smp::lock_responsive;

/// Unmap a page of the kernel image, which the bootloader mapped and the
/// manager therefore has no record of. For the stack guards: `unmap_page`
/// refuses them as unmapped, and a guard that is refused is a guard that is
/// not there. No statistics are recorded, since the mapping was never
/// counted when it was made.
pub fn unmap_image_page(virtual_addr: VirtAddr) -> PagingResult<PhysAddr> {
    without_interrupts(|| lock_responsive(&PAGING_MANAGER).unmap_image_page(virtual_addr))
}
