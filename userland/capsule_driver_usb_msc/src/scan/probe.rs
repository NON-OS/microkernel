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

//! Trying one root port: address its device, and bind it when its
//! configuration has a SCSI-transparent Bulk-Only interface.

use crate::descriptors::parse_config;
use crate::disk::{capacity, unit_ready, Disk};
use crate::state::State;
use crate::xhci::{
    address_device, config_descriptor, configure_bulk, control_no_data, disable_slot, enable_slot,
    E_BUSY,
};

pub enum Probe {
    Ours(Disk),
    /// A device of another class; its slot is given back.
    NotOurs,
    /// Another class driver is addressing the port; ask again later.
    Busy,
    /// The device did not answer as a BOT device should.
    Failed,
}

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);

pub fn probe_port(xhci: u32, port: u8, state: &mut State) -> Probe {
    let Ok(slot) = enable_slot(xhci) else { return Probe::Failed };
    let probe = bind(xhci, slot, port, state);
    if !matches!(probe, Probe::Ours(_)) {
        disable_slot(xhci, slot);
    }
    probe
}

fn bind(xhci: u32, slot: u8, port: u8, state: &mut State) -> Probe {
    match address_device(xhci, slot, port) {
        Ok(()) => {}
        Err(E_BUSY) => return Probe::Busy,
        Err(_) => return Probe::Failed,
    }
    let mut desc = [0u8; 255];
    let Ok(n) = config_descriptor(xhci, slot, &mut desc) else { return Probe::Failed };
    let Ok(found) = parse_config(&desc[..n]) else { return Probe::NotOurs };
    state.install_bindings(&found);
    let b = found.bindings[0];
    if control_no_data(xhci, slot, SET_CONFIGURATION, desc[5] as u16, 0).is_err()
        || configure_bulk(xhci, slot, &b).is_err()
    {
        return Probe::Failed;
    }
    let mut disk = Disk::bound(xhci, slot, &b);
    let Ok((blocks, block_len)) = unit_ready(&disk, state).and_then(|_| capacity(&disk, state))
    else {
        return Probe::Failed;
    };
    (disk.blocks, disk.block_len) = (blocks, block_len);
    Probe::Ours(disk)
}
