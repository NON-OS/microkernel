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
use super::table::{Capsule, CAPSULES};
use super::unreserve::unmap_reserved;
use crate::memory::iommu::DeviceAddress;

/// Back to denied, not to the identity domain: nothing drives the device now,
/// and the next claim attaches it afresh. The domain goes with the capsule's
/// last device, which denies it everything it still mapped; a reserved range
/// goes sooner, at the detach of the last device naming it.
pub(in crate::hardware::broker) fn detach(pid: u32, device_id: u64) {
    detach_held(&mut CAPSULES.lock(), pid, device_id);
}

/// `detach` with CAPSULES held. Each requester the device was joined under
/// once: its own id, each alias (K11), each stream of a described device; a
/// claim of the same device that raced this release keeps the entries it
/// made. A detach that fails keeps the domain.
pub(super) fn detach_held(all: &mut Vec<Capsule>, pid: u32, device_id: u64) {
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return;
    };
    let (mut done, mut kept) = (Vec::new(), false);
    while let Some(denied) = detach_one(&mut all[pos], device_id, &mut done) {
        kept |= !denied;
    }
    if !kept && all[pos].devices.is_empty() {
        all.remove(pos);
    }
}

/// One of `device_id`'s requesters not yet in `done` out of `c`: false when
/// its detach failed, None when none is left.
fn detach_one(c: &mut Capsule, device_id: u64, done: &mut Vec<DeviceAddress>) -> Option<bool> {
    let i = c.devices.iter().position(|(d, a)| *d == device_id && !done.contains(a))?;
    let (_, address) = c.devices.remove(i);
    done.push(address);
    // Another device behind the same VMD or bridge still needs the id.
    if c.devices.iter().any(|(_, a)| *a == address) {
        // A claim of this same device that raced its release keeps its regions.
        if !c.devices.iter().any(|(d, _)| *d == device_id) {
            unmap_reserved(c, device_id);
        }
        say(b"released; requester id still shared");
        return Some(true);
    }
    if c.domain.detach_device(address).is_err() {
        say(b"detach failed; the domain is kept");
        return Some(false);
    }
    // Denied first, so its RMRR regions (L60) and its last-named reserved
    // ranges leave the domain unused.
    unmap_reserved(c, device_id);
    super::unreserve::release(c, address);
    say(b"released and denied");
    Some(true)
}
