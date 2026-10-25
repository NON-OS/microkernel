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
//! A UAS device's pipes given to the host and the device set to use them:
//! both bulk pairs, streams on SuperSpeed, then its configuration and its
//! UAS alternate. The data pair's binding is what the disk keeps.

use nonos_uas::{Pipe, UasPipes};

use crate::descriptors::MscBinding;
use crate::xhci::{configure_bulk, configure_bulk2, configure_streams, control_no_data};

const SET_CONFIGURATION: (u8, u8) = (0x00, 0x09);
const SET_INTERFACE: (u8, u8) = (0x01, 0x0B);

pub(super) fn set_up(
    xhci: u32,
    slot: u8,
    desc: &[u8],
    p: &UasPipes,
    log2: Option<u8>,
) -> Result<MscBinding, i32> {
    let pair = |i: Pipe, o: Pipe| MscBinding {
        interface: p.interface,
        bulk_in: i.address,
        bulk_out: o.address,
        max_packet_in: i.max_packet,
        max_packet_out: o.max_packet,
        max_burst_in: i.burst,
        max_burst_out: o.burst,
    };
    let (control, data) = (pair(p.status, p.command), pair(p.data_in, p.data_out));
    let streams = |pair: u8, dirs: u8| match log2 {
        Some(n) => configure_streams(xhci, slot, pair, dirs, n).map(|_| ()),
        None => Ok(()),
    };
    configure_bulk(xhci, slot, &control)
        .and_then(|_| configure_bulk2(xhci, slot, &data))
        .and_then(|_| streams(0, 1).and_then(|_| streams(1, 3)))
        .and_then(|_| control_no_data(xhci, slot, SET_CONFIGURATION, desc[5] as u16, 0))
        .and_then(|_| {
            control_no_data(xhci, slot, SET_INTERFACE, p.alt as u16, p.interface as u16)
        })?;
    Ok(data)
}
