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

use alloc::vec::Vec;

use crate::xhci::{port_status, PortSnapshot};

use super::configure_port::configure_port;
use super::constants::{MAX_PORTS, PORTSC_CONNECTED, PORT_CLAIMED, PORT_FREE, TRIES};
use super::types::{HidEndpoint, Outcome};

/// Bind the HID devices on ports no class driver holds. `tries` counts, per
/// port, the tries that came to nothing, and a port is decided once it
/// reaches `TRIES`. Returns the new endpoints and whether a port is still
/// open: held by another class driver for now, or not yet decided.
pub fn enumerate(xhci_port: u32, tries: &mut [u8; 256]) -> (Vec<HidEndpoint>, bool) {
    let empty = PortSnapshot { port_id: 0, owner: 0, portsc_raw: 0 };
    let mut snapshots = [empty; MAX_PORTS];
    let Ok(count) = port_status(xhci_port, &mut snapshots) else {
        return (Vec::new(), true);
    };
    let (mut out, mut open) = (Vec::new(), false);
    for snap in snapshots.iter().take(count).copied() {
        let tried = &mut tries[snap.port_id as usize];
        let connected = snap.portsc_raw & PORTSC_CONNECTED != 0;
        if !connected || *tried >= TRIES || snap.owner == PORT_CLAIMED {
            continue;
        }
        if snap.owner != PORT_FREE {
            open = true;
            continue;
        }
        match configure_port(xhci_port, snap, &mut out) {
            Outcome::Bound | Outcome::NotHid => *tried = TRIES,
            Outcome::Busy => open = true,
            Outcome::Failed => {
                *tried += 1;
                open |= *tried < TRIES;
            }
        }
    }
    (out, open)
}
