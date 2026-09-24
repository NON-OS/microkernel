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

//! Which processes are foreign, and who supervises each one.
//!
//! The pairing is set once at creation and never changes: a supervisor
//! cannot be handed a guest it did not create, and a guest cannot be moved
//! between supervisors. Both directions of every peer operation check this
//! table, so an ordinary capsule that learns a foreign pid can still do
//! nothing with it.

use alloc::vec::Vec;

use spin::RwLock;

struct Entry {
    pid: u32,
    supervisor: u32,
}

static FOREIGN: RwLock<Vec<Entry>> = RwLock::new(Vec::new());

/// Record `pid` as foreign under `supervisor`. False if already recorded.
pub(super) fn insert(pid: u32, supervisor: u32) -> bool {
    let mut t = FOREIGN.write();
    if t.iter().any(|e| e.pid == pid) {
        return false;
    }
    t.push(Entry { pid, supervisor });
    true
}

pub fn supervisor_of(pid: u32) -> Option<u32> {
    FOREIGN.read().iter().find(|e| e.pid == pid).map(|e| e.supervisor)
}

/// Every guest of `supervisor`, so its death can take them with it.
pub(super) fn guests_of(supervisor: u32) -> Vec<u32> {
    FOREIGN.read().iter().filter(|e| e.supervisor == supervisor).map(|e| e.pid).collect()
}

/// Drop `pid` from the table, whether it was a guest or a supervisor.
/// Called from process teardown; a supervisor leaving takes its guests.
pub fn clear(pid: u32) {
    let orphans = guests_of(pid);
    FOREIGN.write().retain(|e| e.pid != pid);
    for guest in orphans {
        super::trap_reply::abandon(guest);
    }
}
