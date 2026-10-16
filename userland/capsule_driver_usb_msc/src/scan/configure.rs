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

//! Addressing a port's device on its slot, reading its configuration, and
//! configuring it when it has a SCSI-transparent Bulk-Only interface; its
//! serial number is read for its identity (scan/identity.rs).

use super::asking::Asking;
use super::probe::Probe;
use super::report::{first_interface, Stage};
use crate::descriptors::parse_config;
use crate::disk::{max_lun, serial, Disk};
use crate::state::State;
use crate::xhci::{address_device, config_descriptor, configure_bulk, control_no_data, E_BUSY};

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);

pub(super) fn configure(xhci: u32, slot: u8, port: u8, state: &mut State) -> Probe {
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
    // The first logical unit with a medium ready is served: on a card
    // reader, the slot with the card in it.
    let disk = Disk { port, ..Disk::bound(xhci, slot, &b) };
    Probe::Bound(Asking::start(disk, max_lun(&disk), serial(&disk)))
}
