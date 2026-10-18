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

//! The speed of the device on a root port, which the packet size of a
//! stream depends on.

use nonos_usbnet::xhci::{call, E_IO};

use super::isoch::DATA_AT;
use super::portsc::{connected_in, speed_in};

const OP_PORT_STATUS: u16 = 0x0003;
/// 255 ports of 8 bytes and the 4-byte head.
const DATA_MAX: usize = 4 + 255 * 8;

pub fn port_speed(xhci: u32, port: u8) -> Result<u8, i32> {
    let mut resp = [0u8; DATA_AT + DATA_MAX];
    let len = call(xhci, OP_PORT_STATUS, &[], &mut resp)?;
    speed_in(&resp[DATA_AT..DATA_AT + len.min(DATA_MAX)], port).ok_or(E_IO)
}

/// Whether a device is still on `port`; `None` when driver.xhci0 did not
/// answer, which is not taken for the device gone.
pub fn port_connected(xhci: u32, port: u8) -> Option<bool> {
    let mut resp = [0u8; DATA_AT + DATA_MAX];
    let len = call(xhci, OP_PORT_STATUS, &[], &mut resp).ok()?;
    Some(connected_in(&resp[DATA_AT..DATA_AT + len.min(DATA_MAX)], port))
}
