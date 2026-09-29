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

//! Preempting a lower band for a woken High or RealTime process.

use crate::process::nonos_core::{Priority, ProcessControlBlock, ProcessState};
use crate::process::nonos_core::{CURRENT_PID, PROCESS_TABLE};
use core::sync::atomic::{AtomicU32, Ordering};

/*
 * Selection takes the High band first, but a switch only happens when the
 * running process blocks, yields or spends its ten-tick slice. With every CPU
 * busy, a packet-path capsule woken by the tick sweep or an IPC stayed Ready
 * for up to a slice; a capture showed acknowledgements about every 95 ms.
 *
 * A wake that makes a High or RealTime process Ready records its pid. A tick
 * on a CPU running a lower band takes one record and preempts if that pid is
 * still Ready. A record some other CPU already served, or one that cannot be
 * checked without waiting on a lock, is dropped without a switch. With every
 * slot taken a wake goes unrecorded and runs at the next switch, as before.
 */
const SLOTS: usize = 8;
static WOKEN: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];

fn urgent(pcb: &ProcessControlBlock) -> Option<bool> {
    let prio = *pcb.priority.try_lock()?;
    Some(matches!(prio, Priority::High | Priority::RealTime))
}

/// `pid` just went from Sleeping to Ready.
pub(crate) fn note_ready(pid: u32, pcb: &ProcessControlBlock) {
    if urgent(pcb) == Some(true) {
        let claim =
            |slot: &AtomicU32| slot.compare_exchange(0, pid, Ordering::AcqRel, Ordering::Relaxed);
        WOKEN.iter().map(claim).find(Result::is_ok);
    }
}

/// Whether this CPU should switch now for a woken higher-band process.
pub(super) fn give_way() -> bool {
    if WOKEN.iter().all(|slot| slot.load(Ordering::Relaxed) == 0) {
        return false;
    }
    let running = CURRENT_PID.load(Ordering::Relaxed);
    match PROCESS_TABLE.find_by_pid(running).map(|pcb| urgent(&pcb)) {
        Some(Some(false)) => {}
        _ => return false,
    }
    WOKEN.iter().any(|slot| {
        let pid = slot.swap(0, Ordering::AcqRel);
        pid != 0 && still_ready(pid)
    })
}

fn still_ready(pid: u32) -> bool {
    PROCESS_TABLE
        .find_by_pid(pid)
        .and_then(|pcb| pcb.state.try_lock().map(|s| *s == ProcessState::Ready))
        .unwrap_or(false)
}
