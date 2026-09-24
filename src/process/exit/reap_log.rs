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

//! Exit statuses that outlive the process control block.

use alloc::collections::BTreeMap;
use spin::Mutex;

use crate::process::core::Pid;

static REAP_LOG: Mutex<BTreeMap<Pid, (Pid, i32)>> = Mutex::new(BTreeMap::new());
const REAP_LOG_CAP: usize = 64;

/// Keep `pid`'s status for its parent, evicting to stay inside the cap.
pub(super) fn record(pid: Pid, parent: Pid, code: i32) {
    let mut log = REAP_LOG.lock();
    if log.len() >= REAP_LOG_CAP {
        if let Some((&oldest, _)) = log.iter().next() {
            log.remove(&oldest);
        }
    }
    log.insert(pid, (parent, code));
}

pub(crate) fn reap_exit_status(pid: Pid) -> Option<i32> {
    REAP_LOG.lock().remove(&pid).map(|(_, code)| code)
}

pub(crate) fn reap_exit_status_for(pid: Pid, parent: Pid) -> Option<i32> {
    let mut log = REAP_LOG.lock();
    match log.get(&pid) {
        Some(&(logged_parent, code)) if logged_parent == parent => {
            log.remove(&pid);
            Some(code)
        }
        _ => None,
    }
}

/// Drop what a freshly allocated pid would inherit: its own status, and any
/// status naming it as the parent entitled to read one.
pub(super) fn purge_for_new_pid(pid: Pid) {
    let mut log = REAP_LOG.lock();
    log.remove(&pid);
    log.retain(|_, &mut (parent, _)| parent != pid);
}
