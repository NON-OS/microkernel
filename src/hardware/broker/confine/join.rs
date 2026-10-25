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
//! A device into the claiming capsule's domain, once it has one and nothing
//! it reaches is held elsewhere (one.rs): its RMRR regions mapped one to one
//! (L60), a requester id shared behind a VMD joined to the capsule that holds
//! it, and its reserved ranges in before it is.

extern crate alloc;

use alloc::vec::Vec;

use super::refuse::{refuse, Refusal};
use super::reserve::reserve;
use super::reserve_rmrr::map_reserved;
use super::say::say;
use super::shared::regions_elsewhere;
use super::table::Capsule;
use super::unreserve::{release, unmap_reserved};
use crate::memory::iommu::DeviceAddress;

pub(super) fn join(
    all: &mut Vec<Capsule>,
    pos: usize,
    device_id: u64,
    address: DeviceAddress,
) -> Result<(), Refusal> {
    let others = regions_elsewhere(all, pos);
    map_reserved(&mut all[pos], device_id, address, &others);
    // Drives behind one VMD share its requester id: one capsule may hold
    // several of them in its domain (another is refused in one.rs).
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        all[pos].devices.push((device_id, address));
        say(b"confined to its capsule's domain (shared requester id)");
        return Ok(());
    }
    // Its reserved ranges in the new domain before it is attached there, so
    // it never runs without them; all of them, or the claim is refused.
    if let Err(why) = reserve(&mut all[pos], address) {
        unmap_reserved(&mut all[pos], device_id);
        return refuse(all, pos, why, Refusal::Refused);
    }
    // Out of its bring-up domain first. Between the two writes the device has
    // no context entry, which denies it: the safe side to be on.
    let _ = all[pos].domain.detach_device(address);
    if all[pos].domain.attach_device(address).is_err() {
        release(&mut all[pos], address);
        unmap_reserved(&mut all[pos], device_id);
        return refuse(all, pos, b"refused: attach failed", Refusal::Refused);
    }
    all[pos].devices.push((device_id, address));
    say(b"confined to its capsule's domain");
    Ok(())
}
