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

//! One DMA requester into its capsule's domain, its reserved regions mapped
//! first (L60's RMRRs, C-41's ranges from the tree) so its first request
//! after the move already lands.

use super::attach::Refused;
use super::capsule::capsule_for;
use super::refuse::refuse;
use super::reserve::reserve;
use super::reserve_rmrr::map_reserved;
use super::say::say;
use super::shared::{held_elsewhere, regions_elsewhere};
use super::table::CAPSULES;
use super::unreserve::{release, unmap_reserved};
use crate::memory::iommu::DeviceAddress;

/// Attach `address`, for broker device `device_id`, to `pid`'s domain. With
/// no unit in service a PCI function goes ahead unconfined, said; with
/// `must` (a device the description puts behind a unit) it is refused.
pub(super) fn attach_one(
    pid: u32,
    device_id: u64,
    address: DeviceAddress,
    must: bool,
) -> Result<(), Refused> {
    let mut all = CAPSULES.lock();
    let pos = match capsule_for(&mut all, pid, device_id, address, must)? {
        Some(pos) => pos,
        None => return Ok(()),
    };
    if let Some(why) = held_elsewhere(&all, pos, address) {
        return refuse(&mut all, pos, why, pid, address);
    }
    let others = regions_elsewhere(&all, pos);
    map_reserved(&mut all[pos], pid, device_id, address, &others);
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        all[pos].devices.push((device_id, address));
        say(b"confined to its capsule's domain (shared requester id)", pid, address);
        return Ok(());
    }
    // Its reserved ranges in the new domain before it is attached there, so
    // it never runs without them; all of them, or the claim is refused.
    if let Err(why) = reserve(&mut all[pos], address) {
        unmap_reserved(&mut all[pos], device_id);
        return refuse(&mut all, pos, why, pid, address);
    }
    // Out of the identity domain first. Between the two writes the device has
    // no context entry, which denies it: the safe side to be on.
    let _ = all[pos].domain.detach_device(address);
    if all[pos].domain.attach_device(address).is_err() {
        release(&mut all[pos], address);
        unmap_reserved(&mut all[pos], device_id);
        return refuse(&mut all, pos, b"refused: attach failed", pid, address);
    }
    all[pos].devices.push((device_id, address));
    say(b"confined to its capsule's domain", pid, address);
    Ok(())
}
