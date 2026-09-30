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

//! One pass over the connected ports, each decided or left open.

use alloc::vec::Vec;

use super::probe::{probe_port, Probe};
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::{Port, PORT_CLAIMED, PORT_FREE};

/// Failed binds of one port before it is given up.
pub(super) const TRIES: u8 = 3;

/// The device, if a port holds it, and whether any port is still open: in
/// another class driver's hands for now, or failed fewer than `TRIES` times.
/// A port is closed once `tries` reaches `TRIES`.
pub(super) fn pass(
    xhci: u32,
    ports: Vec<Port>,
    tries: &mut [u8; 256],
    state: &mut State,
) -> (Option<Disk>, bool) {
    let mut open = false;
    for port in ports {
        let tries = &mut tries[port.id as usize];
        if *tries >= TRIES || port.owner == PORT_CLAIMED {
            continue;
        }
        if port.owner != PORT_FREE {
            open = true;
            continue;
        }
        match probe_port(xhci, port.id, state) {
            Probe::Ours(disk) => return (Some(disk), open),
            Probe::NotOurs => *tries = TRIES,
            Probe::Busy => open = true,
            Probe::Failed => {
                *tries += 1;
                open |= *tries < TRIES;
            }
        }
    }
    (None, open)
}
