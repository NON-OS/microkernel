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

//! Looking at every port of one hub: a device that left is forgotten, a new
//! one is brought up. The hub's own change bits say a device was swapped
//! between two looks; only this driver reads them, so they are not lost.

use crate::hub::{clear_port_feature, hub_port, port_status, speed_name, Hub, C_PORT_CONNECTION};

use super::attach_child::attach_child;
use super::constants::TRIES;
use super::devices::Devices;
use super::forget_slot::forget_slot;

pub(super) fn scan_hub(xhci_port: u32, hub: &mut Hub, devs: &mut Devices) {
    for port in 1..=hub.ports {
        let i = port as usize - 1;
        let Some(st) = port_status(xhci_port, hub.slot, port, hub.place.superspeed()) else {
            continue;
        };
        if st.connect_changed {
            let _ = clear_port_feature(xhci_port, hub.slot, port, C_PORT_CONNECTION);
        }
        if hub.child[i] != 0 && (!st.connected || st.connect_changed) {
            forget_slot(xhci_port, hub.child[i], devs);
            hub.child[i] = 0;
            hub_port(hub.slot, port).text(b"device unplugged, its slot is given back").say();
        }
        if !st.connected {
            hub.tries[i] = 0;
            continue;
        }
        if hub.child[i] != 0 || hub.tries[i] >= TRIES {
            continue;
        }
        match attach_child(xhci_port, hub, port, devs) {
            Ok(kept) => {
                hub.tries[i] = TRIES;
                hub.child[i] = kept.unwrap_or(0);
                let what: &[u8] = if kept.is_some() {
                    b" device bound"
                } else {
                    b" device, not a keyboard, mouse or hub; left alone"
                };
                hub_port(hub.slot, port).text(speed_name(st.speed)).text(what).say();
            }
            Err(e) => {
                hub.tries[i] += 1;
                hub_port(hub.slot, port).text(b"stopped: ").text(e.says()).say();
            }
        }
    }
}
