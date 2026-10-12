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
//! pipes given to the host, then its configuration set.

use super::probe::Probe;
use super::ready_unit::finish;
use super::report::{first_interface, Stage};
use crate::descriptors::parse_config;
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::{configure_bulk, control_no_data};

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);

pub(super) fn bot(xhci: u32, slot: u8, desc: &[u8], state: &mut State) -> Probe {
    // A UAS-only device at SuperSpeed waits for streams (0x24): it is not
    // ours yet, and its first interface (08h 06h 62h) says why on the log.
    let Ok(found) = parse_config(desc) else {
        return Probe::NotOurs(first_interface(desc));
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
    finish(Disk::bound(xhci, slot, &b), state)
}
