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

use nonos_libc::mk_uptime_ms;

use crate::xhci::{port_status, PortSnapshot};

use super::configure_port::configure_port;
use super::constants::{MAX_PORTS, PORTSC_CONNECTED, PORT_FREE, TRIES};
use super::devices::Devices;
use super::forget::forget;
use super::root_ports::RootPorts;
use super::types::Outcome;

/// Bind the HID devices on the ports no class driver holds,
/// adding them to `devs`, and forget the ports whose device left. A port is
/// decided once its tries that came to nothing reach `TRIES`, and an empty
/// port starts again from none; whatever comes, it is tried only as often
/// as its budget allows.
pub fn enumerate(xhci_port: u32, roots: &mut RootPorts, devs: &mut Devices) {
    let empty = PortSnapshot { port_id: 0, owner: 0, portsc_raw: 0 };
    let mut snapshots = [empty; MAX_PORTS];
    let Ok(count) = port_status(xhci_port, &mut snapshots) else {
        return;
    };
    let now = mk_uptime_ms().max(0) as u64;
    for snap in snapshots.iter().take(count).copied() {
        let id = snap.port_id as usize;
        let tried = &mut roots.tries[id];
        if snap.portsc_raw & PORTSC_CONNECTED == 0 {
            forget(xhci_port, snap.port_id, tried, devs);
            continue;
        }
        if *tried >= TRIES || snap.owner != PORT_FREE || !roots.budget[id].spend(now) {
            continue;
        }
        match configure_port(xhci_port, snap, devs) {
            Outcome::Bound | Outcome::NotHid => *tried = TRIES,
            Outcome::Busy => {}
            Outcome::Failed => *tried += 1,
        }
    }
}
