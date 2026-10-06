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
use super::report::{Report, Stage};
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::{Port, PORT_CLAIMED, PORT_FREE};

/// Failed binds of one port before it is given up.
pub(super) const TRIES: u8 = 3;

/// A port another class driver holds, or a device of another class.
pub(super) const NOT_OURS: u8 = u8::MAX;

/// The device, if a port holds it, and whether any port is still open: in
/// another class driver's hands at present, or failed fewer than `TRIES` times.
/// A port is closed once `tries` reaches `TRIES`. `report` is left saying
/// what the pass found, and why a port it tried was not taken.
pub(super) fn pass(
    xhci: u32,
    ports: Vec<Port>,
    tries: &mut [u8; 256],
    state: &mut State,
    report: &mut Report,
) -> (Option<Disk>, bool) {
    let mut open = false;
    if ports.is_empty() {
        *report = Report::at(Stage::NoPort, 0, 0);
    }
    report.connected = ports.len().min(u8::MAX as usize) as u8;
    report.closed = ports.iter().filter(|p| tries[p.id as usize] >= TRIES).count() as u8;
    for port in ports {
        let tries = &mut tries[port.id as usize];
        if *tries >= TRIES || port.owner == PORT_CLAIMED {
            continue;
        }
        if port.owner != PORT_FREE {
            open = true;
            continue;
        }
        let (connected, closed) = (report.connected, report.closed);
        match probe_port(xhci, port.id, state) {
            Probe::Ours(disk) => {
                let bound = Report::at(Stage::Bound, port.id, 0);
                *report = Report { aux: disk.block_len, blocks: disk.blocks, ..bound };
                report.connected = connected;
                return (Some(disk), open);
            }
            Probe::NotOurs(class) => {
                *tries = NOT_OURS;
                *report = Report { aux: class, ..Report::at(Stage::NotStorage, port.id, 0) };
            }
            Probe::Busy => {
                open = true;
                *report = Report::at(Stage::Busy, port.id, 0);
            }
            Probe::Failed(stage, e) => {
                *tries += 1;
                open |= *tries < TRIES;
                *report = Report::at(stage, port.id, e);
            }
        }
        (report.connected, report.closed) = (connected, closed);
    }
    (None, open)
}
