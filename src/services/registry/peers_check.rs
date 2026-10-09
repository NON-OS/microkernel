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

use super::held::{endpoint_admits, inbox_admits};
use super::lookup_service;
use super::peers::{may_reach, peers_of};

/*
 * A thread is its process: it runs in the same address space with the same
 * capabilities, so it is held to what its process is held to, under its
 * process's name, and owns what its process owns. Under its own name every
 * thread was "thread", which no peer list names, so a thread went where its
 * process may not; and owning nothing, a worker was refused the drivers its
 * process drives.
 */
fn caller_group() -> Option<u32> {
    let pid = crate::process::current_pid()?;
    crate::process::with_process(pid, |pcb| pcb.thread_group_id())
        .filter(|&tgid| tgid != 0)
        .or(Some(pid))
}

fn caller_name() -> Option<alloc::string::String> {
    let group = caller_group()?;
    crate::process::with_process(group, |pcb| pcb.name())
}

/// Whether the calling process owns the endpoint called `name`.
fn caller_owns(name: &str) -> bool {
    let Some(group) = caller_group() else {
        return false;
    };
    lookup_service(name).is_some_and(|ep| ep.pid == group)
}

/// Whether the caller may send to the endpoint called `target`. A caller
/// with no name is held to nothing it could be named in, but an endpoint
/// held to its services is held whoever asks.
pub fn caller_may_reach(target: &str) -> bool {
    if !endpoint_admits(target, caller_owns) {
        let pid = crate::process::current_pid().unwrap_or(0);
        crate::log::warn!("[IPC-DENY] pid={} ipc to {}: held to the services that drive it", pid, target);
        return false;
    }
    caller_name().is_none_or(|name| may_reach(&name, target))
}

/// Whether the caller, holding the capability bits `held`, may write
/// straight into `dest`'s own inbox: the gate of every endpoint `dest`
/// serves, since all of them are read from that inbox.
pub fn caller_may_write_inbox(dest: u32, held: u64) -> bool {
    let served = super::endpoints_of(dest);
    inbox_admits(served.iter().map(|ep| (ep.name.as_str(), ep.caps_required)), held, caller_owns)
}

/// Whether the caller may send straight to `dest`'s own inbox: only when
/// `dest` serves one of the endpoints on the caller's list.
pub fn caller_may_reach_pid(dest: u32) -> bool {
    let Some(name) = caller_name() else {
        return true;
    };
    let Some(peers) = peers_of(&name) else {
        return true;
    };
    peers.iter().any(|p| lookup_service(p).is_some_and(|ep| ep.pid == dest))
}
