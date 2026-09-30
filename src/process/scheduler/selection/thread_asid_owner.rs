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

//! Which process owns the tables a thread runs on, and so whose address-space
//! id the thread runs under.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::memory::paging::manager::{get_process_cr3, lookup_asid_for_process};
use crate::process::core::ProcessControlBlock;
use crate::process::nonos_core::PROCESS_TABLE;

static UNRESOLVED: AtomicU32 = AtomicU32::new(0);

/// The id of the process whose tables are `tables`: the thread's group
/// leader or parent first, then any process at all.
pub(super) fn owner_asid(pcb: &ProcessControlBlock, tables: u64) -> Option<u32> {
    let near = [pcb.tgid.load(Ordering::Relaxed), pcb.parent_pid()];
    near.iter()
        .find_map(|&o| owned_by(o, tables))
        .or_else(|| PROCESS_TABLE.get_all_processes().iter().find_map(|p| owned_by(p.pid, tables)))
}

fn owned_by(owner: u32, tables: u64) -> Option<u32> {
    if owner == 0 || get_process_cr3(owner) != Some(tables) {
        return None;
    }
    lookup_asid_for_process(owner)
}

/// Should not happen: every thread's tables have an owner in the table. The
/// kernel sentinel it falls back to makes this CPU's own changes flush every
/// CPU, but a change made elsewhere will not reach this one, so it is named.
pub(super) fn note_unresolved(pid: u32) {
    if UNRESOLVED.fetch_add(1, Ordering::Relaxed) < 4 {
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[SMP] no asid for thread pid ").dec(pid as u64);
        l.end();
    }
}
