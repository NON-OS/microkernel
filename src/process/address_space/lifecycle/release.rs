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

use crate::kernel_core::surface_registry::attach_map::forget_pid;
use crate::kernel_core::surface_registry::pin::kept::take_kept;
use crate::kernel_core::surface_registry::pin::orphans::reclaim;
use crate::process::core::ProcessControlBlock;

use super::holder::tables_to_free;

pub fn release(pcb: &Arc<ProcessControlBlock>) {
    let mut mem = pcb.memory_state();
    mem.vmas.clear();
    mem.resident_pages.store(0, Ordering::Release);
    drop(mem);
    // Reclaim the user frames and page tables through the dying process's own
    // ASID. The per-VMA unmap that used to live here walked the active page
    // table, so it freed whatever address space happened to be current; the
    // ASID-scoped teardown frees the leaf frames as well, so it is the only
    // path that touches the right tables. A thread with none runs on tables
    // another thread owns, and that thread's release frees them.
    let Some(asid) = tables_to_free(pcb) else { return };
    // Shared surface frames another process still maps stay allocated
    // (`surface_registry::pin::exit`), whichever thread froze the list.
    let keep = take_kept(pcb.cr3.load(Ordering::Acquire));
    /*
     * A Linux guest's supervisor reads and writes the guest's frames from its
     * own CPU, so no CPU being on these tables does not mean nobody is using
     * them. They are freed with no such copy in progress (`peer_lock.rs`).
     */
    let cleaned = crate::process::foreign::without_peer_calls(|| {
        crate::memory::paging::manager::cleanup_address_space_keeping(asid, &keep)
    });
    if cleaned.is_err() {
        crate::sys::serial::println(b"[EXIT] address_space_cleanup_failed");
    }
    // The group's views went with its tables: only now are its attach
    // records forgotten and the orphans it held last freed (L46).
    forget_pid(crate::hardware::broker::holder_of(pcb.pid));
    reclaim();
}
