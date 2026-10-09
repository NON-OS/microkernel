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

use super::super::super::constants::PAGE_SIZE;
use super::super::super::error::{BuddyAllocError, BuddyAllocResult};
use super::release::release;
use crate::memory::addr::VirtAddr;

pub fn free_pages(addr: VirtAddr, count: usize) -> BuddyAllocResult<()> {
    release(addr, count, true)
}

pub fn free_aligned(addr: VirtAddr, size: usize) -> BuddyAllocResult<()> {
    let page_count = size.checked_add(PAGE_SIZE - 1).ok_or(BuddyAllocError::Overflow)? / PAGE_SIZE;
    free_pages(addr, page_count)
}

pub fn deallocate_pages(addr: VirtAddr, count: usize) -> BuddyAllocResult<()> {
    release(addr, count, false)
}

pub fn deallocate_aligned(addr: VirtAddr, size: usize) -> BuddyAllocResult<()> {
    let page_count = size.checked_add(PAGE_SIZE - 1).ok_or(BuddyAllocError::Overflow)? / PAGE_SIZE;
    deallocate_pages(addr, page_count)
}
