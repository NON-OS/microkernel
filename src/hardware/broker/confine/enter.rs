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

//! A claimed device entering its capsule's domain, its reserved regions
//! mapped first (L60) so its first request after the move already lands.

extern crate alloc;

use alloc::vec::Vec;

use super::attach::Refused;
use super::reserve::{map_reserved, unmap_reserved};
use super::table::{say, Capsule};
use crate::memory::iommu::DeviceAddress;

pub(super) fn enter(
    all: &mut Vec<Capsule>,
    pos: usize,
    pid: u32,
    device_id: u64,
    address: DeviceAddress,
) -> Result<(), Refused> {
    map_reserved(&mut all[pos], pid, device_id, address);
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
        unmap_reserved(&mut all[pos], device_id);
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refused);
    }
    all[pos].devices.push((device_id, address));
    say(b"confined to its capsule's domain", pid, address);
    Ok(())
}
