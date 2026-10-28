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

//! One frame out over the bulk OUT pipe behind its header, and a stalled
//! pipe brought back before its error goes up, so the next transfer finds
//! it working.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::link::Ax;
use super::tx_header::{framed, header, TX_HEADER};

/// One frame of 14 to 1514 bytes out; any other length is refused here,
/// whatever the caller checked, since `tx` holds a header, a full frame
/// and a byte.
pub(super) fn send<B: Bus>(ax: &mut Ax<B>, frame: &[u8]) -> Result<(), i32> {
    if !(ETH_HEADER..=ETH_FRAME_MAX).contains(&frame.len()) {
        return Err(E_INVAL);
    }
    let (n, padded) = framed(frame.len(), ax.pipes.max_packet_out);
    let body = TX_HEADER + frame.len();
    ax.tx[..TX_HEADER].copy_from_slice(&header(frame.len(), padded));
    ax.tx[TX_HEADER..body].copy_from_slice(frame);
    ax.tx[body..n].fill(0);
    match ax.bus.bulk_out(&ax.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut ax.bus, e, ax.pipes.bulk_out)),
    }
}

pub(super) fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
