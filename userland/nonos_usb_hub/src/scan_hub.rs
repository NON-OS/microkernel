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
//! between two looks; only this walk reads them, so they are not lost.

use alloc::vec::Vec;

use crate::attach_child::attach_child;
use crate::classes::Classes;
use crate::forget_slot::forget_slot;
use crate::host::Host;
use crate::line::hub_port;
use crate::node::{Hub, TRIES};
use crate::place::speed_name;
use crate::read::port_status;
use crate::request::{clear_port_feature, C_PORT_CONNECTION};

pub(crate) fn scan_hub<H: Host>(
    h: &mut H,
    hub: &mut Hub,
    hubs: &mut Vec<Hub>,
    classes: &mut impl Classes<H>,
) {
    for port in 1..=hub.ports {
        let i = port as usize - 1;
        let Some(st) = port_status(h, hub.slot, port, hub.place.superspeed()) else {
            continue;
        };
        if st.connect_changed {
            let _ = clear_port_feature(h, hub.slot, port, C_PORT_CONNECTION);
        }
        if hub.child[i] != 0 && (!st.connected || st.connect_changed) {
            forget_slot(h, hub.child[i], hubs, classes);
            hub.child[i] = 0;
            hub_port(hub.slot, port).text(b"device unplugged, its slot is given back").say(h);
        }
        if !st.connected {
            hub.tries[i] = 0;
            continue;
        }
        if hub.child[i] != 0 || hub.tries[i] >= TRIES {
            continue;
        }
        match attach_child(h, hub, port, hubs, classes) {
            Ok(kept) => {
                hub.tries[i] = TRIES;
                hub.child[i] = kept.unwrap_or(0);
                let what: &[u8] = if kept.is_some() {
                    b" device bound"
                } else {
                    b" device, nothing bound; left alone"
                };
                hub_port(hub.slot, port).text(speed_name(st.speed)).text(what).say(h);
            }
            Err(e) => {
                hub.tries[i] += 1;
                hub_port(hub.slot, port).text(b"stopped: ").text(e.says()).say(h);
            }
        }
    }
}
