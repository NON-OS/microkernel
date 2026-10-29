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

//! The gates of a send straight into a process's own inbox, applied to the
//! calling process (peers_check.rs).

use super::held::{endpoint_admits, inbox_admits};
use super::lookup_service;
use super::peers::peers_of;
use super::peers_check::{caller_name, caller_owns};

/// Whether the caller, holding the capability bits `held`, may write
/// straight into `dest`'s own inbox: the gate of every endpoint `dest`
/// serves, since all of them are read from that inbox. A run-time hold that
/// refuses is said as a send by name says it.
pub fn caller_may_write_inbox(dest: u32, held: u64) -> bool {
    let served = super::endpoints_of(dest);
    let gates = served.iter().map(|ep| (ep.name.as_str(), ep.caps_required, ep.held_to.as_deref()));
    if inbox_admits(gates, held, caller_owns) {
        return true;
    }
    let refused = served.iter().find(|ep| {
        ep.held_to.is_some() && !endpoint_admits(&ep.name, ep.held_to.as_deref(), caller_owns)
    });
    if let Some(ep) = refused {
        let pid = crate::process::current_pid().unwrap_or(0);
        let to = ep.held_to.as_deref().unwrap_or("");
        crate::log::warn!(
            "[IPC-DENY] pid={} ipc to pid {}: {} is held to {}",
            pid,
            dest,
            ep.name,
            to
        );
    }
    false
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
