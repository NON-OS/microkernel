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

//! A claim refused for its isolation group: another capsule holds a
//! group member or a requester-id alias, and the line names the member, the
//! hop it reaches the device past, and the process that holds it.

extern crate alloc;

use alloc::string::String;

use super::acs_refusal::{alias_line, refusal_line};
use super::member::Member;
use super::port_only::blocker;
use super::say::say;
use super::table::Capsule;
use crate::memory::iommu::DeviceAddress;

/// Whether a capsule other than `all[pos]` holds a member that blocks the
/// claim (port_only.rs) or an alias of the device; said when one does.
pub(super) fn group_held(
    all: &[Capsule],
    pos: usize,
    (group, aliases): (&[Member], &[DeviceAddress]),
) -> bool {
    let claim_port_only = all[pos].port_only;
    for (_, c) in all.iter().enumerate().filter(|&(i, _)| i != pos) {
        let holds_id = |id: u64| c.devices.iter().any(|(d, _)| *d == id);
        if let Some(m) = blocker(group, holds_id, claim_port_only, c.port_only) {
            say(refusal_line(m.at, m.hop, &holder(c.pid)).as_bytes());
            return true;
        }
        if let Some(a) = aliases.iter().find(|a| c.devices.iter().any(|(_, d)| d == *a)) {
            let at = a.bdf().unwrap_or((0, 0, 0));
            say(alias_line(at, &holder(c.pid)).as_bytes());
            return true;
        }
    }
    false
}

/// The holder's process name, never its pid.
fn holder(pid: u32) -> String {
    crate::process::with_process(pid, |p| p.name.lock().clone())
        .unwrap_or_else(|| String::from("another capsule"))
}
