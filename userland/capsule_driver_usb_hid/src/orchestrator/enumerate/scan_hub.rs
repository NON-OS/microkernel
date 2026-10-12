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

//! Looking at the ports of one hub: a device that left is forgotten, and a
//! new one is brought up a step at a time (`hub::Bringup`). With
//! `busy_only`, only the ports mid way through are looked at, which the
//! poll loop does between its reads; but the first look once a new hub's
//! power is good takes every port, as no rescan comes while another device
//! streams reports. The hub's own change bits say a device was swapped
//! between two looks; only this driver reads them, so they are not lost.
//! Whether any port is still mid way through is returned.

use nonos_libc::mk_uptime_ms;

use crate::hub::{clear_port_feature, hub_port, port_status, Hub, HubError, C_PORT_CONNECTION};

use super::devices::Devices;
use super::forget_slot::forget_slot;
use super::step_port::step_port;

pub(super) fn scan_hub(xhci_port: u32, hub: &mut Hub, devs: &mut Devices, busy_only: bool) -> bool {
    // Powered, but not good yet: looked at again soon.
    if (mk_uptime_ms().max(0) as u64) < hub.ready_ms {
        return true;
    }
    let every_port = !busy_only || hub.ready_ms != 0;
    hub.ready_ms = 0;
    let mut busy = false;
    for port in 1..=hub.ports {
        let i = port as usize - 1;
        if !every_port && !hub.bring[i].busy() {
            continue;
        }
        let Some(st) = port_status(xhci_port, hub.slot, port, hub.place.superspeed()) else {
            // A port that cannot be read ends a try under way.
            if hub.bring[i].busy() {
                hub.bring[i].end_try();
                hub_port(hub.slot, port).text(HubError::PortStatus.says()).say();
            }
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
        if hub.child[i] == 0 {
            step_port(xhci_port, hub, port, st, devs);
        }
        busy |= hub.bring[i].busy();
    }
    busy
}
