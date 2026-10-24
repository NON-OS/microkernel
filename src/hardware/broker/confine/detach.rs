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

extern crate alloc;

use alloc::vec::Vec;

use super::say::say;
use super::table::CAPSULES;
use super::unreserve::unmap_reserved;
use crate::memory::iommu::DeviceAddress;

/// Back to denied, not to the identity domain: nothing drives the device now,
/// and the next claim attaches it afresh. The domain goes with the capsule's
/// last device, which denies it everything it still mapped; a reserved range
/// goes sooner, at the detach of the last device naming it.
pub(in crate::hardware::broker) fn detach(pid: u32, device_id: u64) {
    // Each of its requesters once: a described device may have several
    // streams, and a claim of the same device that raced this release keeps
    // the entries it made.
    let mut done: Vec<DeviceAddress> = Vec::new();
    while detach_one(pid, device_id, &mut done) {}
}

/// Detach one of `device_id`'s requesters not yet in `done`; false when none
/// is left.
fn detach_one(pid: u32, device_id: u64, done: &mut Vec<DeviceAddress>) -> bool {
    let mut all = CAPSULES.lock();
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return false;
    };
    let mine = |(d, a): &(u64, DeviceAddress)| *d == device_id && !done.contains(a);
    let Some(i) = all[pos].devices.iter().position(mine) else {
        return false;
    };
    let (_, address) = all[pos].devices.remove(i);
    done.push(address);
    // Another device behind the same VMD still needs the shared id.
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        // A claim of this same device that raced its release keeps its regions.
        if !all[pos].devices.iter().any(|(d, _)| *d == device_id) {
            unmap_reserved(&mut all[pos], device_id);
        }
        say(b"released; requester id still shared", pid, address);
        return true;
    }
    if all[pos].domain.detach_device(address).is_err() {
        say(b"detach failed; the domain is kept", pid, address);
        return true;
    }
    // Denied first, so its RMRR regions (L60) and its last-named reserved
    // ranges leave the domain unused.
    unmap_reserved(&mut all[pos], device_id);
    super::unreserve::release(&mut all[pos], address);
    say(b"released and denied", pid, address);
    if all[pos].devices.is_empty() {
        all.remove(pos);
    }
    true
}
