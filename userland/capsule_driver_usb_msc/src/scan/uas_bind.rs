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
//! A UAS device bound at high speed: its UAS alternate selected, command
//! and status on the first bulk pair and its data pipes on the second
//! (OP_CONFIGURE_BULK2). At SuperSpeed UAS moves on streams, which no host
//! serves yet (0x24), so the device is bound by BOT there when it offers
//! BOT, and not at all when it does not.

use nonos_uas::{uas_pipes, Pipe};

use super::probe::Probe;
use super::ready_unit::finish;
use super::report::Stage;
use crate::descriptors::MscBinding;
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::{configure_bulk, configure_bulk2, control_no_data, port_speed};

const SPEED_HIGH: u8 = 3;
const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);
const SET_INTERFACE: (u8, u8) = (0x01, 0x0B);

/// `None` when the device is not one to bind by UAS here.
pub(super) fn uas(xhci: u32, slot: u8, port: u8, desc: &[u8], state: &mut State) -> Option<Probe> {
    let p = uas_pipes(desc)?;
    if port_speed(xhci, port) != Some(SPEED_HIGH) {
        return None;
    }
    let pair = |i: Pipe, o: Pipe| MscBinding {
        interface: p.interface,
        bulk_in: i.address,
        bulk_out: o.address,
        max_packet_in: i.max_packet,
        max_packet_out: o.max_packet,
        ..MscBinding::default()
    };
    let (control, data) = (pair(p.status, p.command), pair(p.data_in, p.data_out));
    let on = configure_bulk(xhci, slot, &control)
        .and_then(|_| configure_bulk2(xhci, slot, &data))
        .and_then(|_| control_no_data(xhci, slot, SET_CONFIGURATION, desc[5] as u16, 0))
        .and_then(|_| control_no_data(xhci, slot, SET_INTERFACE, p.alt as u16, p.interface as u16));
    if let Err(e) = on {
        return Some(Probe::Failed(Stage::Configure, e));
    }
    let mut disk = Disk::bound(xhci, slot, &data);
    disk.uas = true;
    Some(finish(disk, state))
}
