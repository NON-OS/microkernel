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

//! The facts `sys_cap_revoke` reads about the caller and the target before
//! `grant_rule::revoke_decision` decides. A process is its thread group, as
//! everywhere else the kernel names an owner (`services::registry`).

use crate::process::with_process;
use crate::services::lookup_service;

/// The services the system cannot run without. Only init may revoke from
/// the process that owns one of these names.
const PROTECTED: [&str; 14] = [
    "compositor",
    "input_router",
    "policy",
    "net.core",
    "net.l2",
    "net.ip",
    "net.udp",
    "net.tcp",
    "net.dns",
    "net.dhcp.client",
    "net.sockets",
    "net.nym",
    "net.anon",
    "net.socks5",
];

/// Init is pid 1, the first process the kernel creates
/// (`process::core::table::inherit` relies on the same).
pub(super) const INIT: u32 = 1;

/// The thread group a pid belongs to, or None when there is no such process.
pub(super) fn group_of(pid: u32) -> Option<u32> {
    with_process(pid, |pcb| pcb.thread_group_id()).map(|g| if g == 0 { pid } else { g })
}

/// The parent of a group's leader.
pub(super) fn parent_of(group: u32) -> Option<u32> {
    with_process(group, |pcb| pcb.parent_pid())
}

/// Whether the group owns a protected service name.
pub(super) fn protected(group: u32) -> bool {
    PROTECTED.iter().any(|name| lookup_service(name).is_some_and(|ep| ep.pid == group))
}
