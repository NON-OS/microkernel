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

use super::report::{first_interface, Stage};
use super::serve::serve_first_ready;
use crate::descriptors::parse_config;
use crate::disk::{Disk, Pause};
use crate::state::State;
use crate::xhci::{
    address_device, config_descriptor, configure_bulk, control_no_data, disable_slot, enable_slot,
    E_BUSY,
};

pub enum Probe {
    Ours(Disk),
    /// A device of another class, its first interface's class, subclass
    /// and protocol (`report::first_interface`); its slot is given back.
    NotOurs(u32),
    /// Another class driver is addressing the port; ask again later.
    Busy,
    /// The device did not answer as a BOT device should, at this stage,
    /// with this errno.
    Failed(Stage, i32),
}

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);

pub fn probe_port(xhci: u32, port: u8, state: &mut State, pause: Pause) -> Probe {
    let slot = match enable_slot(xhci) {
        Ok(slot) => slot,
        Err(e) => return Probe::Failed(Stage::Address, e),
    };
    let probe = bind(xhci, slot, port, state, pause);
    if !matches!(probe, Probe::Ours(_)) {
        disable_slot(xhci, slot);
    }
    probe
}

fn bind(xhci: u32, slot: u8, port: u8, state: &mut State, pause: Pause) -> Probe {
    match address_device(xhci, slot, port) {
        Ok(()) => {}
        Err(E_BUSY) => return Probe::Busy,
        Err(e) => return Probe::Failed(Stage::Address, e),
    }
    let mut desc = [0u8; 255];
    let n = match config_descriptor(xhci, slot, &mut desc) {
        Ok(n) => n,
        Err(e) => return Probe::Failed(Stage::Config, e),
    };
    let Ok(found) = parse_config(&desc[..n]) else {
        return Probe::NotOurs(first_interface(&desc[..n]));
    };
    state.install_bindings(&found);
    let b = found.bindings[0];
    // The controller is given the endpoints before the device is told to
    // use them (xHCI 1.2 section 4.3.5, as Linux's usb_set_configuration
    // reserves bandwidth before it sends SET_CONFIGURATION): a device that
    // starts its bulk pipes on SET_CONFIGURATION finds them there.
    if let Err(e) = configure_bulk(xhci, slot, &b)
        .and_then(|_| control_no_data(xhci, slot, SET_CONFIGURATION, desc[5] as u16, 0))
    {
        return Probe::Failed(Stage::Configure, e);
    }
    serve_first_ready(Disk::bound(xhci, slot, &b), state, pause)
}
