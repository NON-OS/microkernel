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

//! Looking at the ports of every hub held, or with `busy_only` at those mid
//! way through bringing up a device. Each hub is taken out of the list while
//! its ports are looked at, so a hub found below it can be added, and put
//! back after. Whether any port is still mid way through is returned.

use alloc::vec::Vec;

use super::devices::Devices;
use super::scan_hub::scan_hub;

pub fn scan_hubs(xhci_port: u32, devs: &mut Devices, busy_only: bool) -> bool {
    let slots: Vec<u8> = devs.hubs.iter().map(|h| h.slot).collect();
    let mut busy = false;
    for slot in slots {
        let Some(i) = devs.hubs.iter().position(|h| h.slot == slot) else {
            continue;
        };
        let mut hub = devs.hubs.swap_remove(i);
        busy |= scan_hub(xhci_port, &mut hub, devs, busy_only);
        devs.hubs.push(hub);
    }
    busy
}
