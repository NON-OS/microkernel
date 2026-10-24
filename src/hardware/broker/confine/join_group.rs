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

//! A claim refused for its isolation group (K11b), both ways (H-03); the
//! line names the device, its hop, and the process that holds it.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::acs_refusal::{alias_line, peer_alias_line, refusal_line, unreadable_line};
use super::gates::{peer, Peer};
use super::member::Member;
use super::port_only::{blocked_by, Blocked};
use super::say::say;
use super::table::Capsule;
use crate::memory::iommu::DeviceAddress;

/// Whether a capsule other than `all[pos]` holds a device that blocks the
/// claim of `claimed` (port_only.rs `blocked_by`), or a requester-id alias
/// either way; said when one does.
pub(super) fn group_held(
    all: &[Capsule],
    pos: usize,
    (claimed, address): (u64, DeviceAddress),
    (group, aliases): (&[Member], &[DeviceAddress]),
) -> bool {
    let claim_port_only = all[pos].port_only;
    for (_, c) in all.iter().enumerate().filter(|&(i, _)| i != pos) {
        let held: Vec<u64> = c.devices.iter().map(|(d, _)| *d).collect();
        let peers: Vec<(u64, Peer)> = held.iter().filter_map(|&e| Some((e, peer(e)?))).collect();
        let place =
            |e: u64| peers.iter().find(|(id, _)| *id == e).map(|(_, p)| (p.at, p.group.as_deref()));
        let ports = (claim_port_only, c.port_only);
        if let Some(b) = blocked_by(claimed, group, &held, ports, place) {
            let line = match b {
                Blocked::Reaches(at, hop) => refusal_line(at, hop, &holder(c.pid)),
                Blocked::Unreadable(at) => unreadable_line(at, &holder(c.pid)),
            };
            say(line.as_bytes());
            return true;
        }
        if let Some(a) = aliases.iter().find(|a| c.devices.iter().any(|(_, d)| d == *a)) {
            let at = a.bdf().unwrap_or((0, 0, 0));
            say(alias_line(at, &holder(c.pid)).as_bytes());
            return true;
        }
        if let Some((_, p)) = peers.iter().find(|(_, p)| p.aliases.contains(&address)) {
            say(peer_alias_line(p.at, &holder(c.pid)).as_bytes());
            return true;
        }
    }
    false
}

/// The holder's process name, never its pid (K10).
fn holder(pid: u32) -> String {
    crate::process::with_process(pid, |p| p.name.lock().clone())
        .unwrap_or_else(|| String::from("another capsule"))
}
