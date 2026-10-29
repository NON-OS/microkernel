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

//! Who a frame is believed from. The kernel holds linux.serve.<capsule> to
//! the process owning the capsule's endpoint, and records each sender's
//! true pid; a frame counts here only from that owner, checked as the
//! input pump checks its router (wayland/input/pump.rs). A process that
//! ends loses its endpoint (src/process/exit/finalize.rs).

use nonos_app_skeleton::discover::lookup_service;

use super::served::Served;
use crate::linux::net::sock;

/// How often an open connection's opener is looked for.
const LOOK_MS: u64 = 1_000;

/// Whether `sender` owns the endpoint `name`: the pid last seen owning it,
/// kept in `known`, or the registry asked again when that differs.
pub fn owns(known: &mut u32, name: &[u8], sender: u32) -> bool {
    if *known != 0 && *known == sender {
        return true;
    }
    let Some(owner) = lookup_service(name) else {
        return false;
    };
    *known = owner.pid;
    owner.pid == sender
}

/// The connection closed once its opener no longer owns the endpoint: when
/// the owner's frame comes from another pid, as one process owns it, or
/// when the registry, asked every LOOK_MS, names another or none.
pub(super) fn look(s: &mut Served, now: u64, owner: Option<u32>) {
    let Some(pid) = s.server.conn.as_ref().map(|c| c.pid) else { return };
    let asked = owner.is_none() && now.saturating_sub(s.looked) >= LOOK_MS;
    if asked {
        s.looked = now;
    }
    let name = s.field.name().as_bytes();
    let gone = match owner {
        Some(owner) => owner != pid,
        None => asked && !lookup_service(name).is_some_and(|o| o.pid == pid),
    };
    if gone {
        sock::with(|t| s.server.close(t));
    }
}
