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

//! Per-process counters in fixed slots keyed by pid, like the tick table.
//! A pid past the slot count shares a slot with another, which the tick
//! table accepts too; pids are handed out well below it in practice.

use core::sync::atomic::{AtomicU64, Ordering};

use super::kind::Kind;

const SLOTS: usize = 256;

static COUNTS: [[AtomicU64; Kind::COUNT]; SLOTS] =
    [const { [const { AtomicU64::new(0) }; Kind::COUNT] }; SLOTS];

/// One process's counters, read in one pass.
#[derive(Clone, Copy, Default)]
pub struct Snapshot {
    pub syscalls: u64,
    pub ipc_tx: u64,
    pub ipc_rx: u64,
    pub faults: u64,
    pub switches: u64,
    pub user_ticks: u64,
}

fn slot(pid: u32) -> &'static [AtomicU64; Kind::COUNT] {
    &COUNTS[pid as usize % SLOTS]
}

/// Count one event for `pid`. Pid zero is the kernel before any process
/// exists and is not a process, so it is not charged.
#[inline]
pub fn bump(pid: u32, kind: Kind) {
    if pid != 0 {
        slot(pid)[kind as usize].fetch_add(1, Ordering::Relaxed);
    }
}

pub fn snapshot(pid: u32) -> Snapshot {
    let s = slot(pid);
    let at = |k: Kind| s[k as usize].load(Ordering::Relaxed);
    Snapshot {
        syscalls: at(Kind::Syscall),
        ipc_tx: at(Kind::IpcTx),
        ipc_rx: at(Kind::IpcRx),
        faults: at(Kind::Fault),
        switches: at(Kind::Switch),
        user_ticks: at(Kind::UserTick),
    }
}

/// Reset at teardown, so a reused pid starts from nothing.
pub fn clear(pid: u32) {
    for c in slot(pid) {
        c.store(0, Ordering::Relaxed);
    }
}
