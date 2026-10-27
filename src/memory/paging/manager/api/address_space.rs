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

//! Making, handing over and taking address spaces: the manager's table and
//! its directory change together, under the lock. Lookups are in lookup.rs.

use super::globals::PAGING_MANAGER;
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::paging::error::PagingResult;
use crate::smp::lock_responsive;

/*
 * Every holder of the manager lock masks interrupts: a number drawn for it
 * is served in turn (smp/ticket.rs), and one drawn by code an interrupt has
 * stopped holds up every CPU behind it.
 */
pub fn create_address_space(process_id: u32) -> PagingResult<u32> {
    without_interrupts(|| {
        let mut mgr = lock_responsive(&PAGING_MANAGER);
        if !mgr.is_initialized() {
            mgr.init()?;
        }
        mgr.create_address_space(process_id)
    })
}

pub fn cleanup_address_space(asid: u32) -> PagingResult<()> {
    cleanup_address_space_keeping(asid, &[])
}

pub fn cleanup_address_space_keeping(
    asid: u32,
    keep: &[crate::memory::addr::PhysAddr],
) -> PagingResult<()> {
    let cr3 = without_interrupts(|| lock_responsive(&PAGING_MANAGER).take_address_space(asid))?;
    Ok(cr3.map_or((), |cr3| super::super::address_space::free_taken(cr3, keep)))
}

/// Make `process_id` the owner of the tables `asid` names, so its release is
/// the one that frees them. False when there is no such address space.
pub fn hand_over_address_space(asid: u32, process_id: u32) -> bool {
    without_interrupts(|| {
        let mut mgr = lock_responsive(&PAGING_MANAGER);
        let Some(space) = mgr.address_spaces.get_mut(&asid) else { return false };
        space.process_id = process_id;
        // Absent from a directory that overflowed: lookups then ask the table.
        super::super::directory::hand_over(asid, process_id);
        true
    })
}
