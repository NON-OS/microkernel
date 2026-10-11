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

//! A PCI device into the claiming capsule's domain, once it has one: a
//! requester id shared behind a VMD joins the capsule that holds it, and is
//! refused to any other.

extern crate alloc;

use alloc::vec::Vec;

use super::refuse::Refused;
use super::table::{say, Capsule};
use crate::memory::iommu::DeviceAddress;

pub(super) fn join(
    all: &mut Vec<Capsule>,
    pos: usize,
    pid: u32,
    device_id: u64,
    address: DeviceAddress,
) -> Result<(), Refused> {
    // Drives behind one VMD share its requester id. One capsule may hold
    // several of them in its domain; a second capsule cannot take the id
    // without pulling it out from under the first.
    if all.iter().enumerate().any(|(i, c)| i != pos && c.devices.iter().any(|(_, a)| *a == address))
    {
        say(b"refused: requester id held by another capsule", pid, address);
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refused);
    }
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        all[pos].devices.push((device_id, address));
        say(b"confined to its capsule's domain (shared requester id)", pid, address);
        return Ok(());
    }
    // Out of the identity domain first. Between the two writes the device has
    // no context entry, which denies it: the safe side to be on.
    let _ = all[pos].domain.detach_device(address);
    if all[pos].domain.attach_device(address).is_err() {
        say(b"refused: attach failed", pid, address);
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refused);
    }
    all[pos].devices.push((device_id, address));
    say(b"confined to its capsule's domain", pid, address);
    Ok(())
}
