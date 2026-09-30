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

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::process::core::ProcessControlBlock;

pub fn release(pcb: &Arc<ProcessControlBlock>) {
    let mut mem = pcb.memory_state();
    mem.vmas.clear();
    mem.resident_pages.store(0, Ordering::Release);
    drop(mem);
    // Reclaim the user frames and page tables through the dying process's own
    // ASID. The per-VMA unmap that used to live here walked the active page
    // table, so it freed whatever address space happened to be current; the
    // ASID-scoped teardown frees the leaf frames as well, so it is the only
    // path that touches the right tables.
    let Some(asid) = crate::memory::paging::manager::lookup_asid_for_process(pcb.pid) else {
        return;
    };
    /*
     * A thread runs on its group's tables without owning them, and until it
     * leaves the process table it can still be on a CPU under them, taking
     * its own kill or parked on its kernel stack. Freed when the owner went
     * first, they were reused under a running thread, and a threaded guest's
     * exit triple faulted. They pass to a thread still in the table instead,
     * and the last holder's release frees them.
     */
    if let Some(heir) = holder_after(pcb) {
        if crate::memory::paging::manager::hand_over_address_space(asid, heir.pid) {
            // Its token names the ASID it runs in, which it now owns.
            let _ = crate::process::caps::rebind_address_space(&heir);
            return;
        }
    }
    if crate::memory::paging::manager::cleanup_address_space(asid).is_err() {
        crate::sys::serial::println(b"[EXIT] address_space_cleanup_failed");
    }
}

fn holder_after(pcb: &ProcessControlBlock) -> Option<Arc<ProcessControlBlock>> {
    let tables = pcb.cr3.load(Ordering::Acquire);
    if tables == 0 {
        return None;
    }
    crate::process::core::PROCESS_TABLE
        .get_all_processes()
        .into_iter()
        .find(|p| p.pid != pcb.pid && p.cr3.load(Ordering::Acquire) == tables)
}
