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

//! The address-space id a thread runs under, recorded on its CPU.
//!
//! A TLB shootdown reaches only the CPUs whose recorded id matches the
//! address space being changed. A process switch records it on the way to
//! CR3. A thread does not own the tables it runs on, so the per-pid lookup
//! misses and the arch layer loads CR3 directly, leaving whatever id the CPU
//! last recorded. With a guest's threads on several CPUs, an unmap on one of
//! them then skipped the others, which kept translating through the freed
//! page. So the id is found here, from the process that owns those tables,
//! and recorded before CR3 is loaded, as `switch_address_space` does.

use core::sync::atomic::{fence, Ordering};

use super::thread_asid_owner::{note_unresolved, owner_asid};
use crate::memory::paging::manager::lookup_asid_for_process;
use crate::process::nonos_core::PROCESS_TABLE;
use crate::smp::percpu;

/// Record the id for `pid` if it is a thread, returning the id this CPU had
/// so a switch that does not happen can put it back.
pub(super) fn publish(pid: u32) -> u32 {
    let before = percpu::active_asid();
    if !cfg!(feature = "nonos-smp") {
        return before;
    }
    let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) else { return before };
    let tables = pcb.cr3.load(Ordering::Acquire);
    if tables == 0 || lookup_asid_for_process(pid).is_some() {
        return before;
    }
    match owner_asid(&pcb, tables) {
        Some(asid) => percpu::set_active_asid(asid),
        None => {
            /*
             * Its own changes then reach every CPU; see `note_unresolved`.
             */
            percpu::set_active_asid(percpu::ASID_NONE);
            note_unresolved(pid);
        }
    }
    /*
     * Paired with the broadcaster's fence, as in `switch_address_space`.
     */
    fence(Ordering::SeqCst);
    before
}

/// The switch returned before loading CR3: this CPU still runs `before`.
pub(super) fn restore(before: u32) {
    if cfg!(feature = "nonos-smp") && percpu::active_asid() != before {
        percpu::set_active_asid(before);
        fence(Ordering::SeqCst);
    }
}
