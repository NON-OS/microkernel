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
//! A kept device that is gone: what is bound on it is dropped, a hub takes
//! every device below it along, and the slot is given back to the
//! controller driver.

use alloc::vec::Vec;

use crate::classes::Classes;
use crate::host::Host;
use crate::node::Hub;

pub fn forget_slot<H: Host>(
    h: &mut H,
    slot: u8,
    hubs: &mut Vec<Hub>,
    classes: &mut impl Classes<H>,
) {
    classes.forget(slot);
    if let Some(i) = hubs.iter().position(|x| x.slot == slot) {
        let hub = hubs.swap_remove(i);
        for &child in hub.child.iter().filter(|&&c| c != 0) {
            forget_slot(h, child, hubs, classes);
        }
    }
    h.disable_slot(slot);
}

/// Root port `port` is empty: every hub straight on it is forgotten with
/// all below it. How many there were.
pub fn forget_root<H: Host>(
    h: &mut H,
    port: u8,
    hubs: &mut Vec<Hub>,
    classes: &mut impl Classes<H>,
) -> usize {
    let mut n = 0;
    let top = |hubs: &Vec<Hub>| {
        hubs.iter().find(|x| x.place.root_port == port && x.place.depth == 0).map(|x| x.slot)
    };
    while let Some(slot) = top(hubs) {
        forget_slot(h, slot, hubs, classes);
        n += 1;
    }
    n
}
