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
//! What the hub class needs of a USB host controller driver and of the
//! driver.usb_hub0's look at a host's root ports: a hub found on one is
//! configured and kept for the walk, any other device is given back at
//! once for the class drivers, and a root port that empties or changes
//! takes its hub, and every device below it, along.

use alloc::vec::Vec;

use crate::forget_slot::forget_root;
use crate::host::{Host, E_BUSY};
use crate::node::{Hub, TRIES};
use crate::parker::{as_hub, Parker};
use crate::place::Place;

const CONNECTED: u32 = 1 << 0;
const CONNECT_CHANGE: u32 = 1 << 17;

pub trait RootHost: Host {
    /// OP_ADDRESS_DEVICE on root port `port`: the device's speed.
    fn address_root(&mut self, slot: u8, port: u8) -> Result<u8, i32>;
}

/// One root port as PORT_STATUS lists it: port, owner byte, PORTSC.
pub type Root = (u8, u8, u32);

/// Look at every root port in `roots` once. `tried` counts, per port, the
/// tries that came to nothing; a port is left alone at TRIES until it
/// empties or changes.
pub fn roots<H: RootHost>(h: &mut H, roots: &[Root], hubs: &mut Vec<Hub>, tried: &mut [u8; 256]) {
    for &(port, owner, sc) in roots {
        let t = &mut tried[port as usize];
        let held = hubs.iter().any(|x| x.place.root_port == port && x.place.depth == 0);
        if sc & CONNECTED == 0 || sc & CONNECT_CHANGE != 0 {
            *t = 0;
            if held {
                forget_root(h, port, hubs, &mut Parker);
            }
        }
        let held = hubs.iter().any(|x| x.place.root_port == port && x.place.depth == 0);
        if sc & CONNECTED == 0 || held || owner != 0 || *t >= TRIES {
            continue;
        }
        let Ok(slot) = h.enable_slot() else { continue };
        match h.address_root(slot, port) {
            Ok(speed) => {
                match as_hub(h, slot, Place::root(port, speed)) {
                    Some(hub) => hubs.push(hub),
                    None => h.disable_slot(slot),
                }
                *t = TRIES;
            }
            Err(E_BUSY) => h.disable_slot(slot),
            Err(_) => {
                h.disable_slot(slot);
                *t += 1;
            }
        }
    }
}
