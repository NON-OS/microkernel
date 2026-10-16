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

use spin::Mutex;

use crate::process::core::ProcessControlBlock;

/*
 * Set on a thread once its release has run, so its group's tables are never
 * handed to it. Two finalizes at once each found the other as the heir: the
 * ASID went to a thread already on its way out, and the tables, attach
 * records and kept frames all leaked (L49).
 */
const RELEASED: u64 = 1 << 62;

/* The owner check and the hand-over, which two releases must see in turn. */
static RELEASE: Mutex<()> = Mutex::new(());

/*
 * A thread runs on its group's tables without owning them, and until it
 * leaves the process table it can still be on a CPU under them, taking its
 * own kill or parked on its kernel stack. Freed when the owner went first,
 * they were reused under a running thread, and a threaded guest's exit
 * triple faulted. They pass to a thread still in the table instead, and the
 * last holder's release frees them: the ASID when it is `pcb`'s to free.
 */
pub(super) fn tables_to_free(pcb: &ProcessControlBlock) -> Option<u32> {
    let _turn = crate::smp::lock_responsive(&RELEASE);
    pcb.flags.fetch_or(RELEASED, Ordering::AcqRel);
    let asid = crate::memory::paging::manager::lookup_asid_for_process(pcb.pid)?;
    let Some(heir) = holder_after(pcb) else { return Some(asid) };
    if !crate::memory::paging::manager::hand_over_address_space(asid, heir.pid) {
        return Some(asid);
    }
    // Its token names the ASID it runs in, which it now owns.
    let _ = crate::process::caps::rebind_address_space(&heir);
    None
}

/// Whether `pcb`'s release has run: its `cr3` may name freed tables.
pub(crate) fn released(pcb: &ProcessControlBlock) -> bool {
    pcb.flags.load(Ordering::Acquire) & RELEASED != 0
}

/// Another thread still on `pcb`'s tables, which then pass to it.
fn holder_after(pcb: &ProcessControlBlock) -> Option<Arc<ProcessControlBlock>> {
    let tables = pcb.cr3.load(Ordering::Acquire);
    if tables == 0 {
        return None;
    }
    crate::process::core::PROCESS_TABLE
        .get_all_processes()
        .into_iter()
        .find(|p| p.pid != pcb.pid && !released(p) && p.cr3.load(Ordering::Acquire) == tables)
}
