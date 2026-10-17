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

use super::say::say;
use super::table::CAPSULES;

/// Back to denied, not to the identity domain: nothing drives the device now,
/// and the next claim attaches it afresh. The domain goes with the capsule's
/// last device, which denies it everything it still mapped; a reserved range
/// goes sooner, at the detach of the last device naming it.
pub(in crate::hardware::broker) fn detach(pid: u32, device_id: u64) {
    // Each of its requesters: a described device may have several streams.
    while detach_one(pid, device_id) {}
}

/// Detach one of `device_id`'s requesters; false when none is left.
fn detach_one(pid: u32, device_id: u64) -> bool {
    let mut all = CAPSULES.lock();
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return false;
    };
    let Some(i) = all[pos].devices.iter().position(|(d, _)| *d == device_id) else {
        return false;
    };
    let (_, address) = all[pos].devices.remove(i);
    // Another device behind the same VMD still needs the shared id.
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        say(b"released; requester id still shared", pid, address);
        return true;
    }
    if all[pos].domain.detach_device(address).is_err() {
        say(b"detach failed; the domain is kept", pid, address);
        return true;
    }
    // Out of the domain, so its last-named reserved ranges come out too.
    super::unreserve::release(&mut all[pos], address);
    say(b"released and denied", pid, address);
    if all[pos].devices.is_empty() {
        all.remove(pos);
    }
    true
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
