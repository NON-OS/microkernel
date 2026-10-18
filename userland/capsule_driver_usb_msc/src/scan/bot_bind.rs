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

//! A Bulk-Only device bound: its first SCSI-transparent BOT interface's
//! pipes given to the host, then its configuration set; its serial number is
//! read for its identity (scan/identity.rs), and its logical units are
//! asked after (`ask`).

use super::asking::Asking;
use super::probe::Probe;
use super::report::{first_interface, Stage};
use crate::descriptors::parse_config;
use crate::disk::{max_lun, serial, Disk};
use crate::state::State;
use crate::xhci::{configure_bulk, control_no_data};

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);

pub(super) fn bot(xhci: u32, slot: u8, port: u8, desc: &[u8], state: &mut State) -> Probe {
    // A UAS-only device uas_bind.rs did not take (no streams at SuperSpeed,
    // or a slower port) is not ours: its first interface (08h 06h 62h)
    // says why on the log.
    let Ok(found) = parse_config(desc) else {
        return Probe::NotOurs(first_interface(desc));
    };
    state.install_bindings(&found);
    let b = found.bindings[0];
    // The controller is given the endpoints before the device is told to
    // use them (xHCI 1.2 section 4.3.5: Configure Endpoint before
    // SET_CONFIGURATION, as recalled): a device that starts its bulk pipes
    // on SET_CONFIGURATION finds them there.
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
