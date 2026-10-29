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

//! The peer list, applied to the calling process.

use super::held::endpoint_admits;
use super::lookup_service;
use super::peers::may_reach;

/*
 * A thread is its process: it runs in the same address space with the same
 * capabilities, so it is held to what its process is held to, under its
 * process's name, and owns what its process owns. Under its own name every
 * thread was "thread", which no peer list names, so a thread went where its
 * process may not; and owning nothing, a worker was refused the drivers its
 * process drives.
 */
pub(crate) fn caller_group() -> Option<u32> {
    let pid = crate::process::current_pid()?;
    crate::process::with_process(pid, |pcb| pcb.thread_group_id())
        .filter(|&tgid| tgid != 0)
        .or(Some(pid))
}

pub(super) fn caller_name() -> Option<alloc::string::String> {
    let group = caller_group()?;
    crate::process::with_process(group, |pcb| pcb.name())
}

/// Whether the calling process owns the endpoint called `name`.
pub(crate) fn caller_owns(name: &str) -> bool {
    let Some(group) = caller_group() else {
        return false;
    };
    lookup_service(name).is_some_and(|ep| ep.pid == group)
}

/// Whether the caller may send to the endpoint called `target`. A caller
/// with no name is held to nothing it could be named in, but an endpoint
/// held to its services, or to a capsule at run time, is held whoever asks.
pub fn caller_may_reach(target: &str) -> bool {
    let held_to = lookup_service(target).and_then(|ep| ep.held_to);
    if !endpoint_admits(target, held_to.as_deref(), caller_owns) {
        let pid = crate::process::current_pid().unwrap_or(0);
        let to = held_to.as_deref().unwrap_or("the services that drive it");
        crate::log::warn!("[IPC-DENY] pid={} ipc to {}: held to {}", pid, target, to);
        return false;
    }
    caller_name().is_none_or(|name| may_reach(&name, target))
}
