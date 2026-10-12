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

//! Giving a device in its default state an address: its first eight
//! descriptor bytes read at address 0 for EP0's packet size, then
//! SET_ADDRESS to the slot's number and the 2 ms the device may take to
//! use it (USB 2.0, as recalled). A low-speed device's TDs say so, which
//! below a full-speed hub also has the controller send the preamble.

use super::address::AddrError;
use super::table::Table;
use crate::env::{hold, Clock, Mem};
use crate::facts::bounds::SET_ADDRESS_MS;
use crate::sched::layout::CTRL_BUF;
use crate::sched::td::Target;
use crate::xfer::control::control;

/// The device answering at address 0, `low` speed or full, addressed as
/// `slot` on root port `port`: its speed and EP0's packet size.
pub fn assign<C: Clock, M: Mem>(
    m: &M,
    c: &C,
    t: &mut Table,
    (slot, port): (u8, u8),
    low: bool,
) -> Result<(bool, u16), AddrError> {
    let zero = Target { addr: 0, ep: 0, low_speed: low };
    let get = [0x80, 6, 0, 1, 0, 0, 8, 0];
    control(m, c, zero, 8, get).map_err(AddrError::Control)?;
    let mut d = [0u8; 8];
    m.read_bytes(CTRL_BUF, &mut d);
    let mps0 = match d[7] {
        8 | 16 | 32 | 64 => d[7] as u16,
        _ => 8,
    };
    control(m, c, zero, mps0, [0, 5, slot, 0, 0, 0, 0, 0]).map_err(AddrError::Control)?;
    hold(c, SET_ADDRESS_MS);
    if let Some(dev) = t.dev(slot) {
        (dev.port, dev.low_speed, dev.mps0) = (port, low, mps0);
    }
    Ok((low, mps0))
}
