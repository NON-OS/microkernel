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

use super::lookup_service;
use super::peers::{may_reach, peers_of};

fn caller_name() -> Option<alloc::string::String> {
    let pid = crate::process::current_pid()?;
    crate::process::with_process(pid, |pcb| pcb.name())
}

/// Whether the caller may send to the endpoint called `target`. A caller
/// with no name is held to nothing it could be named in.
pub fn caller_may_reach(target: &str) -> bool {
    caller_name().is_none_or(|name| may_reach(&name, target))
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
