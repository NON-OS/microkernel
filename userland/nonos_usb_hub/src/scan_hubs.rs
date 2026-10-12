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
//! Looking at the ports of every hub held. Each hub is taken out of the
//! list while its ports are looked at, so a hub found below it can be
//! added, and put back after.

use alloc::vec::Vec;

use crate::classes::Classes;
use crate::host::Host;
use crate::node::Hub;
use crate::scan_hub::scan_hub;

pub fn scan_hubs<H: Host>(h: &mut H, hubs: &mut Vec<Hub>, classes: &mut impl Classes<H>) {
    let slots: Vec<u8> = hubs.iter().map(|x| x.slot).collect();
    for slot in slots {
        let Some(i) = hubs.iter().position(|x| x.slot == slot) else {
            continue;
        };
        let mut hub = hubs.swap_remove(i);
        scan_hub(h, &mut hub, hubs, classes);
        hubs.push(hub);
    }
}
