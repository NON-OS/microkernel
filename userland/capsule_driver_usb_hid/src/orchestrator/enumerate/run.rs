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

use crate::xhci::{port_status, PortSnapshot};

use super::configure_port::configure_port;
use super::constants::{MAX_PORTS, PORTSC_CONNECTED, PORT_FREE, TRIES};
use super::devices::Devices;
use super::forget::forget;
use super::types::Outcome;

/// Bind the HID devices and hubs on root ports no class driver holds,
/// adding them to `devs`, and forget the ports whose device left. `tries`
/// counts, per port, the tries that came to nothing, and a port is decided
/// once it reaches `TRIES`; an empty port starts again from none.
pub fn enumerate(xhci_port: u32, tries: &mut [u8; 256], devs: &mut Devices) {
    let empty = PortSnapshot { port_id: 0, owner: 0, portsc_raw: 0 };
    let mut snapshots = [empty; MAX_PORTS];
    let Ok(count) = port_status(xhci_port, &mut snapshots) else {
        return;
    };
    for snap in snapshots.iter().take(count).copied() {
        let tried = &mut tries[snap.port_id as usize];
        if snap.portsc_raw & PORTSC_CONNECTED == 0 {
            forget(xhci_port, snap.port_id, tried, devs);
            continue;
        }
        if *tried >= TRIES || snap.owner != PORT_FREE {
            continue;
        }
        match configure_port(xhci_port, snap, devs) {
            Outcome::Bound | Outcome::NotHid => *tried = TRIES,
            Outcome::Busy => {}
            Outcome::Failed => *tried += 1,
        }
    }
}
