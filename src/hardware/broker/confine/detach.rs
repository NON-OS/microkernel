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

/// Back to denied, not to the identity domain: nothing drives the device now,
/// and the next claim attaches it afresh. The domain goes with the capsule's
/// last device, which denies it everything it still mapped; a reserved range
/// goes sooner, at the detach of the last device naming it.
pub(in crate::hardware::broker) fn detach(pid: u32, device_id: u64) {
    detach_held(&mut CAPSULES.lock(), pid, device_id);
}

/// `detach` with CAPSULES already held: every requester the device was
/// joined under, its own id, each alias and each stream of a device
/// the description names. A detach that fails keeps the domain.
pub(super) fn detach_held(all: &mut Vec<Capsule>, pid: u32, device_id: u64) {
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return;
    };
    let mut kept = false;
    while let Some(i) = all[pos].devices.iter().position(|(d, _)| *d == device_id) {
        let (_, address) = all[pos].devices.remove(i);
        // Another device behind the same VMD or bridge still needs the id.
        if all[pos].devices.iter().any(|(_, a)| *a == address) {
            say(b"released; requester id still shared");
            continue;
        }
        if all[pos].domain.detach_device(address).is_err() {
            say(b"detach failed; the domain is kept");
            kept = true;
            continue;
        }
        // Out of the domain, so its last-named reserved ranges come out too.
        super::unreserve::release(&mut all[pos], address);
        say(b"released and denied");
    }
    if !kept && all[pos].devices.is_empty() {
        all.remove(pos);
    }
}

pub(in crate::hardware::broker) fn detach_all(pid: u32) {
    let held: alloc::vec::Vec<u64> = {
        let all = CAPSULES.lock();
        all.iter()
            .filter(|c| c.pid == pid)
            .flat_map(|c| c.devices.iter().map(|(d, _)| *d))
            .collect()
    };
    for device_id in held {
        detach(pid, device_id);
    }
}
