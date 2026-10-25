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
//! One DMA requester into its capsule's domain. What it reaches past the
//! IOMMU is held to that capsule first: its isolation group and requester-id
//! aliases, and any peer below one switch (L72). Then its reserved
//! regions go in (join.rs), so its first request after the move lands.

use super::capsule::capsule_for;
use super::join_group::group_held;
use super::member::Member;
use super::refuse::{refuse, Refusal};
use super::shared::held_elsewhere;
use super::table::CAPSULES;
use crate::memory::iommu::DeviceAddress;

/// Attach `address`, for broker device `device_id`, to `pid`'s domain, with
/// `reach`, its isolation group and requester-id aliases (gates.rs). With no
/// unit in service a PCI function is refused but claimed port-only
/// or under the development opt-in; with `must`, a device the description
/// puts behind a unit, always; `port_only` as port_only.rs says.
pub(super) fn attach_one(
    pid: u32,
    device_id: u64,
    address: DeviceAddress,
    (must, port_only): (bool, bool),
    reach: (&[Member], &[DeviceAddress]),
) -> Result<(), Refusal> {
    let mut all = CAPSULES.lock();
    let Some(pos) = capsule_for(&mut all, pid, device_id, (must, port_only))? else {
        return Ok(());
    };
    // Said by group_held, naming the member and the holder.
    if group_held(&all, pos, (device_id, address), reach) {
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refusal::GroupHeld);
    }
    if let Some(why) = held_elsewhere(&all, pos, address) {
        return refuse(&mut all, pos, why, Refusal::GroupHeld);
    }
    super::join::join(&mut all, pos, device_id, address)?;
    Ok(super::join_alias::join_aliases(&mut all, pid, device_id, reach.1)?)
}

/// Whether `pid`'s capsule claims port-only (port_only.rs), by what it holds.
pub(super) fn port_only_held(pid: u32) -> bool {
    use crate::capabilities::Capability;
    let has = |c: Capability| crate::process::caps::has(pid, c.bit());
    super::port_only::port_only(has(Capability::Dma), has(Capability::Mmio))
}
