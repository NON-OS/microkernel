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

//! Addressing a device: its port reset and enabled, its first eight
//! descriptor bytes read at address 0 for EP0's packet size, then
//! SET_ADDRESS to the slot's number and the 2 ms the device may take to
//! use it (USB 2.0, as recalled).

use super::table::Table;
use crate::env::{hold, Clock, Io, Mem};
use crate::facts::bounds::SET_ADDRESS_MS;
use crate::hc::port::{reset, valid};
use crate::sched::layout::CTRL_BUF;
use crate::sched::td::Target;
use crate::xfer::control::{control, CtrlError};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AddrError {
    /// Another slot holds the port, or the slot is not enabled.
    Busy,
    BadRequest,
    Port(&'static str),
    Control(CtrlError),
}

/// The device's speed (low or full) and EP0's packet size.
pub fn address<I: Io, C: Clock, M: Mem>(
    io: &mut I,
    c: &C,
    m: &M,
    t: &mut Table,
    (slot, port): (u8, u8),
) -> Result<(bool, u16), AddrError> {
    if !valid(port) || t.dev(slot).is_none_or(|d| d.port != 0) {
        return Err(AddrError::BadRequest);
    }
    if t.holder(port).is_some() {
        return Err(AddrError::Busy);
    }
    let low = reset(io, c, port).map_err(AddrError::Port)?;
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
