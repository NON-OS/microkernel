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

use super::table::{say, Capsule, CAPSULES};

/// Back to denied, not to the identity domain: nothing drives the device now,
/// and the next claim attaches it afresh. The domain goes with the capsule's
/// last device, which denies it everything it still mapped.
pub(in crate::hardware::broker) fn detach(pid: u32, device_id: u64) {
    detach_held(&mut CAPSULES.lock(), pid, device_id);
}

/// `detach` with CAPSULES already held.
pub(super) fn detach_held(all: &mut Vec<Capsule>, pid: u32, device_id: u64) {
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return;
    };
    // The device's own id and each alias it was joined under.
    while let Some(i) = all[pos].devices.iter().position(|(d, _)| *d == device_id) {
        let (_, address) = all[pos].devices.remove(i);
        // Another device behind the same VMD or bridge still needs the id.
        if all[pos].devices.iter().any(|(_, a)| *a == address) {
            say(b"released; requester id still shared");
            continue;
        }
        if all[pos].domain.detach_device(address).is_err() {
            say(b"detach failed; the domain is kept");
            return;
        }
        say(b"released and denied");
    }
    if all[pos].devices.is_empty() {
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
