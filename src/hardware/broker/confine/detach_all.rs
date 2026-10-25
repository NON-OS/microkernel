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

//! Every device a capsule's domain holds let go, at the capsule's exit.

extern crate alloc;

use super::detach::detach;
use super::table::CAPSULES;

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
